use std::{
    sync::{
        mpsc::{self, Receiver, RecvTimeoutError, Sender},
        Arc, Mutex, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use serde::Serialize;

use crate::domain::{
    behavior::BehaviorIntent,
    events::DomainEvent,
    pet_state::PetStateV2,
    pet_v2::PetBrainV2,
};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeHealth {
    Ready,
    Degraded,
    Recovering,
    Error,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PetRuntimeSnapshot {
    pub health: RuntimeHealth,
    pub sequence: u64,
    pub state: PetStateV2,
    pub behavior: Option<BehaviorIntent>,
}

impl PetRuntimeSnapshot {
    fn from_brain(health: RuntimeHealth, sequence: u64, brain: &PetBrainV2) -> Self {
        Self {
            health,
            sequence,
            state: brain.state(),
            behavior: brain.behavior(),
        }
    }
}

#[derive(Clone)]
pub struct RuntimeHandle {
    event_tx: Sender<DomainEvent>,
    snapshot: Arc<RwLock<PetRuntimeSnapshot>>,
    event_subscribers: Arc<Mutex<Vec<Sender<DomainEvent>>>>,
}

impl RuntimeHandle {
    pub fn spawn(tick_interval: Duration) -> Self {
        Self::spawn_with_state(tick_interval, PetStateV2::default())
    }

    pub fn spawn_with_state(tick_interval: Duration, initial_state: PetStateV2) -> Self {
        let (event_tx, event_rx) = mpsc::channel::<DomainEvent>();
        let brain = PetBrainV2::from_state(initial_state);
        let snapshot = Arc::new(RwLock::new(PetRuntimeSnapshot::from_brain(
            RuntimeHealth::Ready,
            0,
            &brain,
        )));
        let snapshot_writer = Arc::clone(&snapshot);
        let event_subscribers = Arc::new(Mutex::new(Vec::<Sender<DomainEvent>>::new()));

        thread::spawn(move || {
            let mut brain = brain;
            let mut sequence = 0_u64;
            let mut last_tick = Instant::now();

            loop {
                let elapsed = last_tick.elapsed();
                let wait = if elapsed >= tick_interval {
                    Duration::ZERO
                } else {
                    tick_interval - elapsed
                };

                match event_rx.recv_timeout(wait) {
                    Ok(event) => {
                        brain.handle_event(event);
                        sequence = sequence.saturating_add(1);
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        if let Ok(mut slot) = snapshot_writer.write() {
                            *slot = PetRuntimeSnapshot::from_brain(
                                RuntimeHealth::Degraded,
                                sequence,
                                &brain,
                            );
                        }
                        break;
                    }
                }

                let now = Instant::now();
                let delta = now.duration_since(last_tick);
                if delta >= tick_interval {
                    let delta_ms = delta.as_millis().min(u64::MAX as u128) as u64;
                    brain.handle_event(DomainEvent::Tick { delta_ms });
                    last_tick = now;
                    sequence = sequence.saturating_add(1);
                }

                if let Ok(mut slot) = snapshot_writer.write() {
                    *slot = PetRuntimeSnapshot::from_brain(
                        RuntimeHealth::Ready,
                        sequence,
                        &brain,
                    );
                }
            }
        });

        Self {
            event_tx,
            snapshot,
            event_subscribers,
        }
    }

    pub fn dispatch(&self, event: DomainEvent) -> Result<(), String> {
        self.event_tx
            .send(event.clone())
            .map_err(|_| "pet runtime event channel is unavailable".to_owned())?;

        if let Ok(mut subscribers) = self.event_subscribers.lock() {
            subscribers.retain(|subscriber| subscriber.send(event.clone()).is_ok());
        }

        Ok(())
    }

    pub fn subscribe_events(&self) -> Receiver<DomainEvent> {
        let (tx, rx) = mpsc::channel();
        if let Ok(mut subscribers) = self.event_subscribers.lock() {
            subscribers.push(tx);
        }
        rx
    }

    pub fn snapshot(&self) -> Result<PetRuntimeSnapshot, String> {
        self.snapshot
            .read()
            .map(|snapshot| snapshot.clone())
            .map_err(|_| "pet runtime snapshot lock is poisoned".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::pet_state::Facing;

    #[test]
    fn runtime_starts_from_supplied_state() {
        let mut state = PetStateV2::default();
        state.bond = 0.66;
        state.facing = Facing::Left;
        let runtime = RuntimeHandle::spawn_with_state(Duration::from_secs(60), state);
        let snapshot = runtime.snapshot().expect("runtime snapshot");
        assert_eq!(snapshot.state.bond, 0.66);
        assert_eq!(snapshot.state.facing, Facing::Left);
    }

    #[test]
    fn runtime_dispatches_domain_events_to_observers() {
        let runtime = RuntimeHandle::spawn(Duration::from_secs(60));
        let events = runtime.subscribe_events();
        runtime
            .dispatch(DomainEvent::PetPetted)
            .expect("dispatch event");

        assert!(matches!(
            events.recv_timeout(Duration::from_secs(1)),
            Ok(DomainEvent::PetPetted)
        ));
    }
}

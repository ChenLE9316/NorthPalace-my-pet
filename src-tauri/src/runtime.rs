use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use crate::domain::events::DomainEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHealth {
    Ready,
    Degraded,
    Recovering,
    Error,
}

#[derive(Debug, Clone)]
pub struct RuntimeSnapshot<T: Clone> {
    pub health: RuntimeHealth,
    pub state: T,
}

/// Lightweight monotonic runtime clock owned by Rust.
///
/// This module is intentionally generic scaffolding for V0.2. Pet Brain wiring will move
/// from the current UI-driven `tick_pet` command into this owner loop next.
pub struct RuntimeClock {
    tick_interval: Duration,
}

impl RuntimeClock {
    pub fn new(tick_interval: Duration) -> Self {
        Self { tick_interval }
    }

    pub fn spawn<F>(self, mut on_event: F) -> thread::JoinHandle<()>
    where
        F: FnMut(DomainEvent) + Send + 'static,
    {
        thread::spawn(move || {
            let mut last = Instant::now();
            loop {
                thread::sleep(self.tick_interval);
                let now = Instant::now();
                let delta_ms = now.duration_since(last).as_millis().min(u64::MAX as u128) as u64;
                last = now;
                on_event(DomainEvent::Tick { delta_ms });
            }
        })
    }
}

/// Shared immutable-snapshot slot for presentation consumers.
/// The future Pet Runtime actor remains the single writer.
pub type SharedSnapshot<T> = Arc<RwLock<T>>;

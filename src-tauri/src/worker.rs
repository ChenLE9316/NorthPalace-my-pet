use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkerHealth {
    Starting,
    Running,
    Stopped,
    Error,
    Panicked,
    Detached,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkerStatus {
    pub name: String,
    pub health: WorkerHealth,
    pub last_error: Option<String>,
}

#[derive(Default)]
struct CancellationState {
    cancelled: Mutex<bool>,
    wake: Condvar,
}

#[derive(Clone, Default)]
pub struct CancellationToken {
    inner: Arc<CancellationState>,
}

impl CancellationToken {
    pub fn is_cancelled(&self) -> bool {
        *self
            .inner
            .cancelled
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn wait_timeout(&self, duration: Duration) -> bool {
        let guard = self
            .inner
            .cancelled
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if *guard {
            return true;
        }

        let (guard, _) = self
            .inner
            .wake
            .wait_timeout_while(guard, duration, |cancelled| !*cancelled)
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *guard
    }

    fn cancel(&self) {
        let mut cancelled = self
            .inner
            .cancelled
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if *cancelled {
            return;
        }
        *cancelled = true;
        self.inner.wake.notify_all();
    }
}

#[derive(Debug)]
struct WorkerState {
    health: WorkerHealth,
    last_error: Option<String>,
}

struct ManagedWorker {
    name: String,
    state: Arc<Mutex<WorkerState>>,
    join: Option<JoinHandle<()>>,
}

#[derive(Default)]
struct SupervisorInner {
    cancellation: CancellationToken,
    workers: Mutex<Vec<ManagedWorker>>,
}

#[derive(Clone, Default)]
pub struct WorkerSupervisor {
    inner: Arc<SupervisorInner>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShutdownReport {
    pub joined: Vec<String>,
    pub detached: Vec<String>,
}

impl WorkerSupervisor {
    pub fn spawn<F>(&self, name: impl Into<String>, worker: F) -> Result<(), String>
    where
        F: FnOnce(CancellationToken) -> Result<(), String> + Send + 'static,
    {
        let name = name.into();
        let mut workers = self
            .inner
            .workers
            .lock()
            .map_err(|_| "worker supervisor registry lock is poisoned".to_owned())?;

        if workers.iter().any(|entry| entry.name == name) {
            return Err(format!("worker already registered: {name}"));
        }
        if self.inner.cancellation.is_cancelled() {
            return Err("worker supervisor is shutting down".to_owned());
        }

        let state = Arc::new(Mutex::new(WorkerState {
            health: WorkerHealth::Starting,
            last_error: None,
        }));
        let state_for_thread = Arc::clone(&state);
        let token = self.inner.cancellation.clone();
        let thread_name = format!("lenvu-{name}");

        let join = thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                update_state(&state_for_thread, WorkerHealth::Running, None);
                let result = catch_unwind(AssertUnwindSafe(|| worker(token)));
                match result {
                    Ok(Ok(())) => {
                        update_state(&state_for_thread, WorkerHealth::Stopped, None);
                    }
                    Ok(Err(error)) => {
                        update_state(&state_for_thread, WorkerHealth::Error, Some(error));
                    }
                    Err(_) => {
                        update_state(
                            &state_for_thread,
                            WorkerHealth::Panicked,
                            Some("worker panicked".to_owned()),
                        );
                    }
                }
            })
            .map_err(|error| format!("failed to spawn worker {name}: {error}"))?;

        workers.push(ManagedWorker {
            name,
            state,
            join: Some(join),
        });
        Ok(())
    }

    pub fn snapshot(&self) -> Vec<WorkerStatus> {
        let Ok(workers) = self.inner.workers.lock() else {
            return Vec::new();
        };

        workers
            .iter()
            .map(|worker| {
                let state = worker
                    .state
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                WorkerStatus {
                    name: worker.name.clone(),
                    health: state.health,
                    last_error: state.last_error.clone(),
                }
            })
            .collect()
    }

    pub fn shutdown_and_join(&self, timeout: Duration) -> ShutdownReport {
        self.inner.cancellation.cancel();

        let mut pending = {
            let Ok(mut workers) = self.inner.workers.lock() else {
                return ShutdownReport {
                    joined: Vec::new(),
                    detached: vec!["worker-registry-lock".to_owned()],
                };
            };

            workers
                .iter_mut()
                .filter_map(|worker| {
                    worker
                        .join
                        .take()
                        .map(|join| (worker.name.clone(), Arc::clone(&worker.state), join))
                })
                .collect::<Vec<_>>()
        };

        let deadline = Instant::now() + timeout;
        let mut joined = Vec::new();

        while !pending.is_empty() && Instant::now() < deadline {
            let mut index = 0;
            while index < pending.len() {
                if pending[index].2.is_finished() {
                    let (name, state, join) = pending.swap_remove(index);
                    if join.join().is_err() {
                        update_state(
                            &state,
                            WorkerHealth::Panicked,
                            Some("worker join observed panic".to_owned()),
                        );
                    }
                    joined.push(name);
                } else {
                    index += 1;
                }
            }

            if !pending.is_empty() {
                thread::sleep(Duration::from_millis(5));
            }
        }

        let detached = pending
            .into_iter()
            .map(|(name, state, _join)| {
                update_state(
                    &state,
                    WorkerHealth::Detached,
                    Some("worker did not stop before shutdown deadline".to_owned()),
                );
                name
            })
            .collect();

        ShutdownReport { joined, detached }
    }
}

fn update_state(state: &Arc<Mutex<WorkerState>>, health: WorkerHealth, last_error: Option<String>) {
    if let Ok(mut state) = state.lock() {
        state.health = health;
        state.last_error = last_error;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    #[test]
    fn cancellation_interrupts_wait() {
        let supervisor = WorkerSupervisor::default();
        let (tx, rx) = mpsc::sync_channel(1);
        supervisor
            .spawn("interruptible-wait", move |token| {
                let started = Instant::now();
                let cancelled = token.wait_timeout(Duration::from_secs(5));
                tx.send((cancelled, started.elapsed()))
                    .map_err(|error| format!("failed to report wait result: {error}"))?;
                Ok(())
            })
            .expect("spawn worker");

        thread::sleep(Duration::from_millis(20));
        let report = supervisor.shutdown_and_join(Duration::from_secs(1));
        let (cancelled, elapsed) = rx
            .recv_timeout(Duration::from_secs(1))
            .expect("wait result");

        assert_eq!(report.joined, vec!["interruptible-wait".to_owned()]);
        assert!(report.detached.is_empty());
        assert!(cancelled);
        assert!(elapsed < Duration::from_secs(1));
    }

    #[test]
    fn supervisor_joins_cooperative_worker() {
        let supervisor = WorkerSupervisor::default();
        supervisor
            .spawn("cooperative", |token| {
                while !token.wait_timeout(Duration::from_secs(60)) {}
                Ok(())
            })
            .expect("spawn worker");

        let report = supervisor.shutdown_and_join(Duration::from_secs(1));
        assert_eq!(report.joined, vec!["cooperative".to_owned()]);
        assert!(report.detached.is_empty());
        assert_eq!(supervisor.snapshot()[0].health, WorkerHealth::Stopped);
    }

    #[test]
    fn supervisor_records_worker_errors() {
        let supervisor = WorkerSupervisor::default();
        supervisor
            .spawn("failing", |_token| Err("expected failure".to_owned()))
            .expect("spawn worker");

        for _ in 0..100 {
            let status = supervisor.snapshot();
            if status
                .first()
                .is_some_and(|status| status.health == WorkerHealth::Error)
            {
                assert_eq!(status[0].last_error.as_deref(), Some("expected failure"));
                return;
            }
            thread::sleep(Duration::from_millis(5));
        }

        panic!("worker error status was not observed");
    }
}

use std::time::{Duration, Instant};

use crate::{
    domain::{
        behavior::BehaviorKind,
        events::DomainEvent,
        pet_state::{Facing, Locomotion},
    },
    runtime::RuntimeHandle,
    worker::WorkerSupervisor,
};

const MOTION_TICK: Duration = Duration::from_millis(40);
const EDGE_MARGIN_PHYSICAL_PX: i32 = 8;
const MONITOR_EDGE_GAP_TOLERANCE_PX: i32 = 16;
const MIN_MONITOR_VERTICAL_OVERLAP_PX: i32 = 64;
const WALK_LOGICAL_PX_PER_SEC: f64 = 55.0;
const RUN_LOGICAL_PX_PER_SEC: f64 = 110.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HorizontalDirection {
    Left,
    Right,
}

impl HorizontalDirection {
    fn sign(self) -> f64 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }

    fn reverse(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn facing(self) -> Facing {
        match self {
            Self::Left => Facing::Left,
            Self::Right => Facing::Right,
        }
    }

    fn from_facing(facing: Facing) -> Self {
        match facing {
            Facing::Left => Self::Left,
            Facing::Right => Self::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RectI32 {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl RectI32 {
    fn right(self) -> i32 {
        self.x
            .saturating_add(self.width.min(i32::MAX as u32) as i32)
    }

    fn bottom(self) -> i32 {
        self.y
            .saturating_add(self.height.min(i32::MAX as u32) as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MonitorGeometry {
    full: RectI32,
    work: RectI32,
}

#[derive(Debug, Clone, Copy)]
struct HorizontalBounds {
    min_x: i32,
    max_x: i32,
    ground_y: i32,
}

fn motion_speed_logical_px_per_sec(locomotion: Locomotion) -> Option<f64> {
    match locomotion {
        Locomotion::Walk => Some(WALK_LOGICAL_PX_PER_SEC),
        Locomotion::Run => Some(RUN_LOGICAL_PX_PER_SEC),
        Locomotion::Stationary | Locomotion::Jump => None,
    }
}

fn allows_monitor_transition(behavior: Option<BehaviorKind>) -> bool {
    matches!(behavior, Some(BehaviorKind::Explore))
}

fn monitor_geometry(monitor: &tauri::window::Monitor) -> MonitorGeometry {
    let position = monitor.position();
    let size = monitor.size();
    let work = monitor.work_area();

    MonitorGeometry {
        full: RectI32 {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        },
        work: RectI32 {
            x: work.position.x,
            y: work.position.y,
            width: work.size.width,
            height: work.size.height,
        },
    }
}

fn calculate_bounds(work: RectI32, window_width: u32, window_height: u32) -> HorizontalBounds {
    let min_x = work.x.saturating_add(EDGE_MARGIN_PHYSICAL_PX);
    let window_width = window_width.min(i32::MAX as u32) as i32;
    let window_height = window_height.min(i32::MAX as u32) as i32;

    let max_x = work
        .right()
        .saturating_sub(window_width)
        .saturating_sub(EDGE_MARGIN_PHYSICAL_PX)
        .max(min_x);
    let ground_y = work
        .bottom()
        .saturating_sub(window_height)
        .saturating_sub(EDGE_MARGIN_PHYSICAL_PX)
        .max(work.y);

    HorizontalBounds {
        min_x,
        max_x,
        ground_y,
    }
}

fn vertical_overlap(a: RectI32, b: RectI32) -> i32 {
    a.bottom()
        .min(b.bottom())
        .saturating_sub(a.y.max(b.y))
        .max(0)
}

fn horizontal_gap(
    current: MonitorGeometry,
    candidate: MonitorGeometry,
    direction: HorizontalDirection,
) -> Option<i32> {
    let gap = match direction {
        HorizontalDirection::Right => {
            if candidate.full.x < current.full.right() {
                return None;
            }
            candidate.full.x.saturating_sub(current.full.right())
        }
        HorizontalDirection::Left => {
            if candidate.full.right() > current.full.x {
                return None;
            }
            current.full.x.saturating_sub(candidate.full.right())
        }
    };

    (gap <= MONITOR_EDGE_GAP_TOLERANCE_PX).then_some(gap)
}

fn find_adjacent_monitor(
    current: MonitorGeometry,
    monitors: impl IntoIterator<Item = MonitorGeometry>,
    direction: HorizontalDirection,
) -> Option<MonitorGeometry> {
    monitors
        .into_iter()
        .filter(|candidate| *candidate != current)
        .filter_map(|candidate| {
            let gap = horizontal_gap(current, candidate, direction)?;
            let overlap = vertical_overlap(current.full, candidate.full);
            if overlap < MIN_MONITOR_VERTICAL_OVERLAP_PX {
                return None;
            }
            Some((candidate, gap, overlap))
        })
        .min_by(|left, right| left.1.cmp(&right.1).then_with(|| right.2.cmp(&left.2)))
        .map(|(candidate, _, _)| candidate)
}

fn projected_x(
    current_x: f64,
    direction: HorizontalDirection,
    speed_physical_px_per_sec: f64,
    delta_seconds: f64,
) -> f64 {
    current_x + direction.sign() * speed_physical_px_per_sec.max(0.0) * delta_seconds.max(0.0)
}

fn reaches_edge(projected: f64, direction: HorizontalDirection, bounds: HorizontalBounds) -> bool {
    match direction {
        HorizontalDirection::Left => projected <= bounds.min_x as f64,
        HorizontalDirection::Right => projected >= bounds.max_x as f64,
    }
}

fn advance_x(
    current_x: f64,
    direction: HorizontalDirection,
    speed_physical_px_per_sec: f64,
    delta_seconds: f64,
    bounds: HorizontalBounds,
) -> (f64, HorizontalDirection) {
    let mut next = projected_x(
        current_x,
        direction,
        speed_physical_px_per_sec,
        delta_seconds,
    );
    let mut next_direction = direction;

    if next <= bounds.min_x as f64 {
        next = bounds.min_x as f64;
        next_direction = direction.reverse();
    } else if next >= bounds.max_x as f64 {
        next = bounds.max_x as f64;
        next_direction = direction.reverse();
    }

    (next, next_direction)
}

fn publish_facing(runtime: &RuntimeHandle, direction: HorizontalDirection) -> Result<(), String> {
    runtime.dispatch(DomainEvent::PetFacingChanged {
        facing: direction.facing(),
    })
}

pub fn spawn_pet_motion_controller(
    window: tauri::WebviewWindow,
    runtime: RuntimeHandle,
    supervisor: &WorkerSupervisor,
) -> Result<(), String> {
    supervisor.spawn("windows-pet-motion", move |token| {
        let mut direction: Option<HorizontalDirection> = None;
        let mut published_direction: Option<HorizontalDirection> = None;
        let mut last_step = Instant::now();
        let mut fractional_x: Option<f64> = None;

        while !token.is_cancelled() {
            let now = Instant::now();
            let delta_seconds = now.duration_since(last_step).as_secs_f64();
            last_step = now;

            let Ok(snapshot) = runtime.snapshot() else {
                if token.wait_timeout(MOTION_TICK) {
                    break;
                }
                continue;
            };

            let Some(speed_logical) = motion_speed_logical_px_per_sec(snapshot.state.locomotion)
            else {
                fractional_x = None;
                direction = None;
                published_direction = None;
                if token.wait_timeout(MOTION_TICK) {
                    break;
                }
                continue;
            };

            let mut active_direction = direction
                .unwrap_or_else(|| HorizontalDirection::from_facing(snapshot.state.facing));
            direction = Some(active_direction);

            if published_direction != Some(active_direction) {
                publish_facing(&runtime, active_direction)?;
                published_direction = Some(active_direction);
            }

            let Ok(Some(monitor)) = window.current_monitor() else {
                if token.wait_timeout(MOTION_TICK) {
                    break;
                }
                continue;
            };
            let window_position = window
                .outer_position()
                .map_err(|error| format!("failed to read pet window position: {error}"))?;
            let window_size = window
                .outer_size()
                .map_err(|error| format!("failed to read pet window size: {error}"))?;
            let Ok(scale_factor) = window.scale_factor() else {
                if token.wait_timeout(MOTION_TICK) {
                    break;
                }
                continue;
            };

            let current_monitor = monitor_geometry(&monitor);
            let bounds = calculate_bounds(
                current_monitor.work,
                window_size.width,
                window_size.height,
            );
            let current_x = fractional_x.unwrap_or(window_position.x as f64);
            let speed_physical = speed_logical * scale_factor.max(0.5);
            let delta_seconds = delta_seconds.min(0.25);
            let projected = projected_x(current_x, active_direction, speed_physical, delta_seconds);

            if reaches_edge(projected, active_direction, bounds)
                && allows_monitor_transition(snapshot.behavior.as_ref().map(|behavior| behavior.kind))
            {
                let adjacent = window.available_monitors().ok().and_then(|monitors| {
                    find_adjacent_monitor(
                        current_monitor,
                        monitors.iter().map(monitor_geometry),
                        active_direction,
                    )
                });

                if let Some(target_monitor) = adjacent {
                    let target_bounds = calculate_bounds(
                        target_monitor.work,
                        window_size.width,
                        window_size.height,
                    );
                    let target_x = match active_direction {
                        HorizontalDirection::Left => target_bounds.max_x,
                        HorizontalDirection::Right => target_bounds.min_x,
                    };
                    fractional_x = Some(target_x as f64);

                    window
                        .set_position(tauri::PhysicalPosition::new(
                            target_x,
                            target_bounds.ground_y,
                        ))
                        .map_err(|error| format!("failed to move pet to adjacent monitor: {error}"))?;

                    if token.wait_timeout(MOTION_TICK) {
                        break;
                    }
                    continue;
                }
            }

            let previous_direction = active_direction;
            let (next_x, next_direction) = advance_x(
                current_x,
                active_direction,
                speed_physical,
                delta_seconds,
                bounds,
            );
            active_direction = next_direction;
            direction = Some(active_direction);
            fractional_x = Some(next_x);

            if active_direction != previous_direction {
                publish_facing(&runtime, active_direction)?;
                published_direction = Some(active_direction);
            }

            let physical_x = next_x.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32;
            if window_position.x != physical_x || window_position.y != bounds.ground_y {
                window
                    .set_position(tauri::PhysicalPosition::new(physical_x, bounds.ground_y))
                    .map_err(|error| format!("failed to move pet window: {error}"))?;
            }

            if token.wait_timeout(MOTION_TICK) {
                break;
            }
        }

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monitor(x: i32, y: i32, width: u32, height: u32) -> MonitorGeometry {
        MonitorGeometry {
            full: RectI32 {
                x,
                y,
                width,
                height,
            },
            work: RectI32 {
                x,
                y,
                width,
                height: height.saturating_sub(40),
            },
        }
    }

    #[test]
    fn work_area_clamps_pet_inside_edges() {
        let bounds = calculate_bounds(
            RectI32 {
                x: 0,
                y: 0,
                width: 1920,
                height: 1040,
            },
            360,
            320,
        );
        assert_eq!(bounds.min_x, 8);
        assert_eq!(bounds.max_x, 1552);
        assert_eq!(bounds.ground_y, 712);
    }

    #[test]
    fn motion_reverses_at_right_edge_without_transition() {
        let bounds = HorizontalBounds {
            min_x: 0,
            max_x: 100,
            ground_y: 0,
        };
        let (x, direction) = advance_x(98.0, HorizontalDirection::Right, 100.0, 0.1, bounds);
        assert_eq!(x, 100.0);
        assert_eq!(direction, HorizontalDirection::Left);
        assert_eq!(direction.facing(), Facing::Left);
    }

    #[test]
    fn initial_direction_respects_domain_facing() {
        assert_eq!(HorizontalDirection::from_facing(Facing::Left), HorizontalDirection::Left);
        assert_eq!(HorizontalDirection::from_facing(Facing::Right), HorizontalDirection::Right);
    }

    #[test]
    fn adjacent_horizontal_monitor_is_selected() {
        let current = monitor(0, 0, 1920, 1080);
        let right = monitor(1920, 100, 2560, 1440);
        let found = find_adjacent_monitor(current, [current, right], HorizontalDirection::Right);
        assert_eq!(found, Some(right));
    }

    #[test]
    fn vertically_stacked_monitor_is_not_a_horizontal_neighbor() {
        let current = monitor(0, 0, 1920, 1080);
        let above = monitor(0, -1080, 1920, 1080);
        let found = find_adjacent_monitor(current, [current, above], HorizontalDirection::Right);
        assert_eq!(found, None);
    }

    #[test]
    fn disconnected_monitor_gap_prevents_autonomous_teleport() {
        let current = monitor(0, 0, 1920, 1080);
        let distant = monitor(2000, 0, 1920, 1080);
        let found = find_adjacent_monitor(current, [current, distant], HorizontalDirection::Right);
        assert_eq!(found, None);
    }

    #[test]
    fn only_ambient_explore_can_cross_monitors() {
        assert!(allows_monitor_transition(Some(BehaviorKind::Explore)));
        assert!(!allows_monitor_transition(Some(BehaviorKind::Play)));
        assert!(!allows_monitor_transition(Some(BehaviorKind::FocusGuard)));
        assert!(!allows_monitor_transition(None));
    }

    #[test]
    fn stationary_and_jump_do_not_translate_window() {
        assert!(motion_speed_logical_px_per_sec(Locomotion::Stationary).is_none());
        assert!(motion_speed_logical_px_per_sec(Locomotion::Jump).is_none());
        assert!(motion_speed_logical_px_per_sec(Locomotion::Walk).is_some());
        assert!(motion_speed_logical_px_per_sec(Locomotion::Run).is_some());
    }
}

use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{
    domain::{events::DomainEvent, pet_state::{Facing, Locomotion}},
    runtime::RuntimeHandle,
};

const MOTION_TICK: Duration = Duration::from_millis(40);
const EDGE_MARGIN_PHYSICAL_PX: i32 = 8;
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

fn calculate_bounds(
    work_x: i32,
    work_y: i32,
    work_width: u32,
    work_height: u32,
    window_width: u32,
    window_height: u32,
) -> HorizontalBounds {
    let min_x = work_x.saturating_add(EDGE_MARGIN_PHYSICAL_PX);
    let work_right = work_x.saturating_add(work_width.min(i32::MAX as u32) as i32);
    let work_bottom = work_y.saturating_add(work_height.min(i32::MAX as u32) as i32);
    let window_width = window_width.min(i32::MAX as u32) as i32;
    let window_height = window_height.min(i32::MAX as u32) as i32;

    let max_x = work_right
        .saturating_sub(window_width)
        .saturating_sub(EDGE_MARGIN_PHYSICAL_PX)
        .max(min_x);
    let ground_y = work_bottom
        .saturating_sub(window_height)
        .saturating_sub(EDGE_MARGIN_PHYSICAL_PX)
        .max(work_y);

    HorizontalBounds { min_x, max_x, ground_y }
}

fn advance_x(
    current_x: f64,
    direction: HorizontalDirection,
    speed_physical_px_per_sec: f64,
    delta_seconds: f64,
    bounds: HorizontalBounds,
) -> (f64, HorizontalDirection) {
    let mut next = current_x
        + direction.sign() * speed_physical_px_per_sec.max(0.0) * delta_seconds.max(0.0);
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

fn publish_facing(runtime: &RuntimeHandle, direction: HorizontalDirection) -> bool {
    runtime
        .dispatch(DomainEvent::PetFacingChanged {
            facing: direction.facing(),
        })
        .is_ok()
}

pub fn spawn_pet_motion_controller(
    window: tauri::WebviewWindow,
    runtime: RuntimeHandle,
) {
    thread::spawn(move || {
        let mut direction = HorizontalDirection::Right;
        let mut published_direction: Option<HorizontalDirection> = None;
        let mut last_step = Instant::now();
        let mut fractional_x: Option<f64> = None;

        loop {
            let now = Instant::now();
            let delta_seconds = now.duration_since(last_step).as_secs_f64();
            last_step = now;

            let Ok(snapshot) = runtime.snapshot() else {
                thread::sleep(MOTION_TICK);
                continue;
            };

            let Some(speed_logical) = motion_speed_logical_px_per_sec(snapshot.state.locomotion) else {
                fractional_x = None;
                thread::sleep(MOTION_TICK);
                continue;
            };

            if published_direction != Some(direction) {
                if !publish_facing(&runtime, direction) {
                    break;
                }
                published_direction = Some(direction);
            }

            let Ok(Some(monitor)) = window.current_monitor() else {
                thread::sleep(MOTION_TICK);
                continue;
            };
            let Ok(window_position) = window.outer_position() else {
                break;
            };
            let Ok(window_size) = window.outer_size() else {
                break;
            };
            let Ok(scale_factor) = window.scale_factor() else {
                thread::sleep(MOTION_TICK);
                continue;
            };

            let work = monitor.work_area();
            let bounds = calculate_bounds(
                work.position.x,
                work.position.y,
                work.size.width,
                work.size.height,
                window_size.width,
                window_size.height,
            );

            let current_x = fractional_x.unwrap_or(window_position.x as f64);
            let speed_physical = speed_logical * scale_factor.max(0.5);
            let previous_direction = direction;
            let (next_x, next_direction) = advance_x(
                current_x,
                direction,
                speed_physical,
                delta_seconds.min(0.25),
                bounds,
            );
            direction = next_direction;
            fractional_x = Some(next_x);

            if direction != previous_direction {
                if !publish_facing(&runtime, direction) {
                    break;
                }
                published_direction = Some(direction);
            }

            let physical_x = next_x.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32;
            if window_position.x != physical_x || window_position.y != bounds.ground_y {
                if window
                    .set_position(tauri::PhysicalPosition::new(physical_x, bounds.ground_y))
                    .is_err()
                {
                    break;
                }
            }

            thread::sleep(MOTION_TICK);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_area_clamps_pet_inside_edges() {
        let bounds = calculate_bounds(0, 0, 1920, 1040, 360, 320);
        assert_eq!(bounds.min_x, 8);
        assert_eq!(bounds.max_x, 1552);
        assert_eq!(bounds.ground_y, 712);
    }

    #[test]
    fn motion_reverses_at_right_edge() {
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
    fn stationary_and_jump_do_not_translate_window() {
        assert!(motion_speed_logical_px_per_sec(Locomotion::Stationary).is_none());
        assert!(motion_speed_logical_px_per_sec(Locomotion::Jump).is_none());
        assert!(motion_speed_logical_px_per_sec(Locomotion::Walk).is_some());
        assert!(motion_speed_logical_px_per_sec(Locomotion::Run).is_some());
    }
}

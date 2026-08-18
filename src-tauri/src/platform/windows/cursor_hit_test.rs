use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use serde::Deserialize;

use crate::worker::WorkerSupervisor;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "shape", rename_all = "snake_case")]
pub enum CursorHitRegion {
    Ellipse { cx: f64, cy: f64, rx: f64, ry: f64 },
    Rect { x: f64, y: f64, width: f64, height: f64 },
}

impl CursorHitRegion {
    fn contains(&self, x: f64, y: f64) -> bool {
        match *self {
            Self::Ellipse { cx, cy, rx, ry } => {
                let dx = (x - cx) / rx;
                let dy = (y - cy) / ry;
                dx * dx + dy * dy <= 1.0
            }
            Self::Rect { x: left, y: top, width, height } => {
                x >= left && x <= left + width && y >= top && y <= top + height
            }
        }
    }

    fn valid(&self) -> bool {
        let finite = |value: f64| value.is_finite();
        match *self {
            Self::Ellipse { cx, cy, rx, ry } => {
                [cx, cy, rx, ry].into_iter().all(finite)
                    && (0.0..=1.0).contains(&cx)
                    && (0.0..=1.0).contains(&cy)
                    && rx > 0.0
                    && ry > 0.0
                    && rx <= 1.0
                    && ry <= 1.0
            }
            Self::Rect { x, y, width, height } => {
                [x, y, width, height].into_iter().all(finite)
                    && (0.0..=1.0).contains(&x)
                    && (0.0..=1.0).contains(&y)
                    && width > 0.0
                    && height > 0.0
                    && x + width <= 1.000_001
                    && y + height <= 1.000_001
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct CursorHitTestHandle {
    regions: Arc<RwLock<Vec<CursorHitRegion>>>,
}

impl CursorHitTestHandle {
    pub fn set_regions(&self, regions: Vec<CursorHitRegion>) -> Result<(), String> {
        if regions.is_empty() {
            return Err("pet hit-test region list cannot be empty".to_owned());
        }
        if !regions.iter().all(CursorHitRegion::valid) {
            return Err("pet hit-test region contains invalid normalized geometry".to_owned());
        }

        let mut slot = self
            .regions
            .write()
            .map_err(|_| "pet hit-test region lock is poisoned".to_owned())?;
        *slot = regions;
        Ok(())
    }

    fn snapshot(&self) -> Vec<CursorHitRegion> {
        self.regions
            .read()
            .map(|regions| regions.clone())
            .unwrap_or_default()
    }
}

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetCursorPos(point: *mut Point) -> i32;
}

fn cursor_position() -> Option<Point> {
    let mut point = Point { x: 0, y: 0 };
    let ok = unsafe { GetCursorPos(&mut point) };
    (ok != 0).then_some(point)
}

pub fn spawn_cursor_passthrough_sensor(
    window: tauri::WebviewWindow,
    hit_test: CursorHitTestHandle,
    supervisor: &WorkerSupervisor,
) -> Result<(), String> {
    supervisor.spawn("windows-cursor-passthrough", move |token| {
        let mut last_ignore: Option<bool> = None;

        while !token.is_cancelled() {
            let regions = hit_test.snapshot();
            let desired_ignore = if regions.is_empty() {
                // Safe startup state: never make the window unreachable before the WebView
                // publishes its semantic hit regions.
                false
            } else {
                let Some(cursor) = cursor_position() else {
                    if token.wait_timeout(Duration::from_millis(30)) {
                        break;
                    }
                    continue;
                };
                let position = window
                    .outer_position()
                    .map_err(|error| format!("failed to read pet window position: {error}"))?;
                let size = window
                    .inner_size()
                    .map_err(|error| format!("failed to read pet window size: {error}"))?;
                if size.width == 0 || size.height == 0 {
                    false
                } else {
                    let local_x = cursor.x - position.x;
                    let local_y = cursor.y - position.y;
                    let inside_window = local_x >= 0
                        && local_y >= 0
                        && local_x < size.width as i32
                        && local_y < size.height as i32;

                    if !inside_window {
                        true
                    } else {
                        let x = local_x as f64 / size.width as f64;
                        let y = local_y as f64 / size.height as f64;
                        !regions.iter().any(|region| region.contains(x, y))
                    }
                }
            };

            if last_ignore != Some(desired_ignore) {
                window
                    .set_ignore_cursor_events(desired_ignore)
                    .map_err(|error| format!("failed to update pet cursor passthrough: {error}"))?;
                last_ignore = Some(desired_ignore);
            }

            if token.wait_timeout(Duration::from_millis(30)) {
                break;
            }
        }

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ellipse_contains_center() {
        let region = CursorHitRegion::Ellipse {
            cx: 0.5,
            cy: 0.5,
            rx: 0.2,
            ry: 0.3,
        };
        assert!(region.contains(0.5, 0.5));
        assert!(!region.contains(0.9, 0.9));
    }

    #[test]
    fn rect_validation_rejects_overflow() {
        let region = CursorHitRegion::Rect {
            x: 0.9,
            y: 0.1,
            width: 0.2,
            height: 0.2,
        };
        assert!(!region.valid());
    }
}

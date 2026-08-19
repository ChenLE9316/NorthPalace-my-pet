mod accessibility;
mod active_window;
mod clock;
mod cursor_hit_test;
mod display;
mod idle;
mod motion;

pub use active_window::spawn_active_window_sensor;
pub use clock::spawn_local_time_sensor;
pub use cursor_hit_test::{CursorHitRegion, CursorHitTestHandle, spawn_cursor_passthrough_sensor};
pub use display::{DisplayContext, read_display_context};
pub use idle::spawn_idle_sensor;
pub use motion::spawn_pet_motion_controller;

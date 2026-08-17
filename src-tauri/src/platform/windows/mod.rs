mod accessibility;
mod active_window;
mod clock;
mod cursor_hit_test;
mod display;
mod idle;
mod motion;

pub use active_window::spawn_active_window_sensor;
pub use clock::spawn_local_time_sensor;
pub use cursor_hit_test::{
    spawn_cursor_passthrough_sensor, CursorHitRegion, CursorHitTestHandle,
};
pub use display::{read_display_context, DisplayContext};
pub use idle::spawn_idle_sensor;
pub use motion::spawn_pet_motion_controller;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalExtent {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalArea {
    pub position: PhysicalPoint,
    pub size: PhysicalExtent,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayContext {
    pub scale_factor: f64,
    pub monitor_name: Option<String>,
    pub monitor_count: usize,
    pub monitor_bounds: Option<PhysicalArea>,
    pub work_area: Option<PhysicalArea>,
    pub window_position: PhysicalPoint,
    pub window_size: PhysicalExtent,
}

pub fn read_display_context(window: &tauri::WebviewWindow) -> Result<DisplayContext, String> {
    let scale_factor = window.scale_factor().map_err(|error| error.to_string())?;
    let monitor_count = window
        .available_monitors()
        .map_err(|error| error.to_string())?
        .len();
    let monitor = window.current_monitor().map_err(|error| error.to_string())?;
    let window_position = window.outer_position().map_err(|error| error.to_string())?;
    let window_size = window.inner_size().map_err(|error| error.to_string())?;

    let (monitor_name, monitor_bounds, work_area) = if let Some(monitor) = monitor {
        let position = monitor.position();
        let size = monitor.size();
        let work = monitor.work_area();

        (
            monitor.name().cloned(),
            Some(PhysicalArea {
                position: PhysicalPoint {
                    x: position.x,
                    y: position.y,
                },
                size: PhysicalExtent {
                    width: size.width,
                    height: size.height,
                },
            }),
            Some(PhysicalArea {
                position: PhysicalPoint {
                    x: work.position.x,
                    y: work.position.y,
                },
                size: PhysicalExtent {
                    width: work.size.width,
                    height: work.size.height,
                },
            }),
        )
    } else {
        (None, None, None)
    };

    Ok(DisplayContext {
        scale_factor,
        monitor_name,
        monitor_count,
        monitor_bounds,
        work_area,
        window_position: PhysicalPoint {
            x: window_position.x,
            y: window_position.y,
        },
        window_size: PhysicalExtent {
            width: window_size.width,
            height: window_size.height,
        },
    })
}

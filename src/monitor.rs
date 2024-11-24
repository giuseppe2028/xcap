use std::fmt::Display;
use image::RgbaImage;

use crate::{error::XCapResult, platform::impl_monitor::ImplMonitor};
use crate::DisplayOptions::DisplayOptions;

#[derive(Debug, Clone)]
pub struct Monitor {
    pub(crate) impl_monitor: ImplMonitor,
}

impl PartialEq<Self> for Monitor {
    fn eq(& self, other: &Self) -> bool {
        self.impl_monitor.id == other.impl_monitor.id
    }
}

impl Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.impl_monitor.name.clone())
    }
}
impl Eq for Monitor{
}
unsafe impl Send for Monitor {}
impl Monitor {
    pub(crate) fn new(impl_monitor: ImplMonitor) -> Monitor {
        Monitor { impl_monitor }
    }
}

impl Monitor {
    pub fn all() -> XCapResult<Vec<Monitor>> {
        let monitors = ImplMonitor::all()?
            .iter()
            .map(|impl_monitor| Monitor::new(impl_monitor.clone()))
            .collect();

        Ok(monitors)
    }

    pub fn from_point(x: i32, y: i32) -> XCapResult<Monitor> {
        let impl_monitor = ImplMonitor::from_point(x, y)?;

        Ok(Monitor::new(impl_monitor))
    }
}

impl Monitor {
    /// Unique identifier associated with the screen.
    pub fn id(&self) -> u32 {
        self.impl_monitor.id
    }
    /// Unique identifier associated with the screen.
    pub fn name(&self) -> &str {
        &self.impl_monitor.name
    }
    /// The screen x coordinate.
    pub fn x(&self) -> i32 {
        self.impl_monitor.x
    }
    /// The screen x coordinate.
    pub fn y(&self) -> i32 {
        self.impl_monitor.y
    }
    /// The screen pixel width.
    pub fn width(&self) -> u32 {
        self.impl_monitor.width
    }
    /// The screen pixel height.
    pub fn height(&self) -> u32 {
        self.impl_monitor.height
    }
    /// Can be 0, 90, 180, 270, represents screen rotation in clock-wise degrees.
    pub fn rotation(&self) -> f32 {
        self.impl_monitor.rotation
    }
    /// Output device's pixel scale factor.
    pub fn scale_factor(&self) -> f32 {
        self.impl_monitor.scale_factor
    }
    /// The screen refresh rate.
    pub fn frequency(&self) -> f32 {
        self.impl_monitor.frequency
    }
    /// Whether the screen is the main screen
    pub fn is_primary(&self) -> bool {
        self.impl_monitor.is_primary
    }
    /// Set the screen x coordinate.
    pub fn set_x(&mut self, x: i32) {
        self.impl_monitor.x = x;
    }

    /// Set the screen y coordinate.
    pub fn set_y(&mut self, y: i32) {
        self.impl_monitor.y = y;
    }

    pub fn set_width(&mut self, width: u32){
        self.impl_monitor.width = width;
    }

    pub fn set_height(&mut self, height: u32){
        self.impl_monitor.height = height;
    }
}

impl Monitor {
    /// Capture image of the monitor
    pub fn capture_image(&self, options:Option<DisplayOptions>) -> XCapResult<RgbaImage> {
        self.impl_monitor.capture_image(options)
    }
}

use glium::glutin::dpi::LogicalSize;

pub struct WindowSetup {
    pub title: String,
    pub icon: Option<String>,
}

impl WindowSetup {
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }
    
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_owned());
        self
    }
}

impl Default for WindowSetup {
    fn default() -> WindowSetup {
        WindowSetup {
           title: "ggwp window".to_owned(),
           icon: None
        }
    }
}

pub struct WindowMode {
    pub dimensions: LogicalSize<f32>,
}

impl WindowMode {
    pub fn dimensions(mut self, width: f32, height: f32) -> Self {
        self.dimensions.width = width;
        self.dimensions.height = height;
        self
    }
}

impl Default for WindowMode {
    fn default() -> WindowMode {
        WindowMode {
            dimensions: LogicalSize::new(600.0, 400.0),
        }
    }
}
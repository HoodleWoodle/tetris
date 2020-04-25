use crate::ggwp::{
    conf::{WindowSetup, WindowMode},
    graphics::{
        text::{FontData, QueuedText},
        DrawCall
    },
};
use std::{
    path::PathBuf,
    time::{Instant, Duration},
    rc::Rc,
    cell::RefCell,
};
use glium::{
    Display, Program,
    glutin::{
        event_loop::EventLoop,
        window::{WindowBuilder, Icon},
        ContextBuilder as OpenGLContextBuilder,
        dpi::PhysicalPosition,
    },
    texture::srgb_texture2d::SrgbTexture2d,
};
use freetype::Library;

pub use error::{GameError, GameResult};

pub mod audio;
pub mod conf;
pub mod event;
pub mod graphics;
pub mod input;
pub mod mint;
pub mod timer;
pub mod util;
pub mod texture_packer;
mod error;

const VERTEX_SHADER: &str = "
    #version 410 core
    layout(location = 0) in vec2 position;
    layout(location = 1) in vec2 uv;
    layout(location = 2) in vec4 color;

    out vec2 pass_uv;
    out vec4 pass_color;

    void main()
    {
        pass_uv = uv;
        pass_color = color;

        gl_Position = vec4(position, 0.0, 1.0);
    }
";
const FRAGMENT_SHADER: &str = "
    #version 410 core

    out vec4 out_color;

    in vec2 pass_uv;
    in vec4 pass_color;

    uniform sampler2D u_texture;

    void main()
    {
        out_color = pass_color * texture(u_texture, pass_uv);
    }
";

pub struct Context<'a> {
    display: Display,

    key_states: [bool; 161],

    ups: usize,
    ticks: usize,
    last_ticks: usize,

    fps: usize,
    frames: usize,
    last_frames: usize,

    duration: Duration,
    last_time: Instant,
    residual_update_dt: Duration,

    ft_lib: Library,
    program: Program,
    textures: Vec<SrgbTexture2d>,
    fonts: Vec<Rc<RefCell<FontData>>>,
    
    draw_calls: Vec<DrawCall<'a>>,
    texts: Vec<QueuedText>,
}

pub struct ContextBuilder {
    window_setup: WindowSetup,
    window_mode: WindowMode,
}

impl ContextBuilder {
    pub fn new(_: &str, _: &str) -> ContextBuilder {
        ContextBuilder {
            window_setup: WindowSetup::default(),
            window_mode: WindowMode::default(),
        }
    }

    pub fn add_resource_path<T>(self, _: T) -> Self
    where
        T: Into<PathBuf>
    {
        self
    }

    pub fn window_setup(mut self, setup: WindowSetup) -> Self {
        self.window_setup = setup;
        self
    }

    pub fn window_mode(mut self, mode: WindowMode) -> Self {
        self.window_mode = mode;
        self
    }
    
    pub fn build<'a>(self) -> GameResult<(Context<'a>, EventLoop<()>)> {
        // event loop
        let event_loop = EventLoop::new();

        let icon = match self.window_setup.icon {
            Some(path) => {
                let rgba = util::image(path)?;
                let width = rgba.width();
                let height = rgba.height();
                let icon = Icon::from_rgba(rgba.into_raw(), width, height)?;
                Some(icon)
            },
            None => None,
        };

        let window_builder = WindowBuilder::new()
            .with_resizable(false)
            .with_title(self.window_setup.title)
            .with_window_icon(icon)
            .with_inner_size(self.window_mode.dimensions)
            .with_visible(false)
            .with_transparent(true);
    
        // OpenGL context
        let context_builder = OpenGLContextBuilder::new();
    
        // display
        let display = Display::new(window_builder, context_builder, &event_loop)?;

        // center window
        let window_size = display.gl_window().window().outer_size();
        let monitor_size = display.gl_window().window().current_monitor().size();
        let x_pos = (monitor_size.width - window_size.width) / 2;
        let y_pos = (monitor_size.height - window_size.height) / 2;
        let position = PhysicalPosition::new(x_pos, y_pos);
        display.gl_window().window().set_outer_position(position);

        // freetype
        let ft_lib = Library::init()?;

        // shader
        let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None)?;

        let ctx = Context {
            display,

            key_states: [false; 161],
            
            ups: 0,
            ticks: 0,
            last_ticks: 0,

            fps: 0,
            frames: 0,
            last_frames: 0,

            duration: Duration::default(),
            last_time: Instant::now(),
            residual_update_dt: Duration::default(),

            ft_lib,
            program,
            textures: vec![],
            fonts: vec![],
            
            draw_calls: vec![],
            texts: vec![]
        };

        ctx.display.gl_window().window().set_visible(true);

        Ok((ctx, event_loop))
    }
}
use crate::ggwp::{
    GameResult, Context,
    mint::Point2,
};
use glium::{
    DrawParameters,
    vertex::VertexBuffer,
    index::IndexBuffer,
};
use serde::Deserialize;

pub mod spritebatch;
pub mod text;
mod img;

pub use img::Image;
pub use text::{
    Font, Text, FilterMode, Scale,
    queue_text, draw_queued_text,
};

pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

#[derive(Copy, Clone, Deserialize)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Color {
    fn combine(mut self, color: Color) -> Color {
        self.r *= color.r;
        self.g *= color.g;
        self.b *= color.b;
        self.a *= color.a;
        self
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
           x,
           y,
           w,
           h
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vtx {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

impl Vtx {
    pub fn new(position: [f32; 2], uv: [f32; 2], color: [f32; 4]) -> Vtx {
        Vtx {
            position,
            uv,
            color,
        }
    }
}

implement_vertex!(Vtx, position, uv, color);

pub struct DrawCall<'a> {
    pub vertices: VertexBuffer<Vtx>,
    pub indices: IndexBuffer<u16>,
    pub texture_id: usize,
    pub draw_parameters: DrawParameters<'a>,
}

impl<'a> DrawCall<'a> {
    fn new(vertices: VertexBuffer<Vtx>, indices: IndexBuffer<u16>, texture_id: usize, draw_parameters: DrawParameters<'a>) -> DrawCall<'a> {
        DrawCall {
            vertices,
            indices,
            texture_id,
            draw_parameters
        }
    }
}

#[derive(Copy, Clone)]
pub struct DrawParam {
    pub src: Rect,
    pub dest: Point2<f32>,
    pub size: Option<Point2<f32>>,
    pub color: Color,
}

impl DrawParam {
    pub fn src(mut self, src: Rect) -> Self {
        self.src = src;
        self
    }

    pub fn dest<P>(mut self, dest: P) -> Self
    where
        P: Into<Point2<f32>>
    {
        self.dest = dest.into();
        self
    }
    
    pub fn size(mut self, size: Option<Point2<f32>>) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for DrawParam {
    fn default() -> DrawParam {
        DrawParam {
            src: Rect::new(0.0, 0.0, 1.0, 1.0),
            dest: Point2 { x: 0.0, y: 0.0 },
            color: WHITE,
            size: None,
        }
    }
}

pub trait Drawable {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()>;
}

pub fn draw<D, T>(ctx: &mut Context, drawable: &D, param: T) -> GameResult<()>
where
    D: Drawable,
    T: Into<DrawParam>
{
    drawable.draw(ctx, param.into())
}

pub fn present(_: &mut Context) -> GameResult<()> {
    Ok(())
}
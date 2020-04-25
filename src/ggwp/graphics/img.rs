use crate::ggwp::{
    GameResult, Context,
    graphics::{Drawable, DrawParam, DrawCall},
    util,
};
use std::path::Path;
use glium::{
    texture::{
        srgb_texture2d::SrgbTexture2d,
        RawImage2d,
    },
    DrawParameters, BackfaceCullingMode, Blend,
    vertex::VertexBuffer,
    index::{PrimitiveType, IndexBuffer},
};

#[derive(Copy, Clone)]
pub struct Image {
    pub id: usize,
}

impl Image {
    pub fn new<P>(ctx: &mut Context, path: P) -> GameResult<Self>
    where
        P: AsRef<Path>
    {
        let loaded = util::image(path)?;
        let dimensions = (loaded.width(), loaded.height());
        let data = loaded.into_raw();

        Image::from_raw(ctx, dimensions, data)
    }

    pub fn from_raw(ctx: &mut Context, dimensions: (u32, u32), data: Vec<u8>) -> GameResult<Self> {
        let texture = SrgbTexture2d::new(&ctx.display, RawImage2d::from_raw_rgba(data, dimensions))?;
        let id = ctx.textures.len();
        ctx.textures.push(texture);

        let image = Image {
            id,
        };

        Ok(image)
    }

    pub fn size(&self, ctx: &mut Context) -> (f32, f32) {
        let texture = &ctx.textures[self.id];

        (texture.get_width() as f32, texture.get_height().unwrap() as f32)
    }
}

impl Drawable for Image {
    fn draw (&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let mut vertices = Vec::with_capacity(4);
        let mut indices = Vec::with_capacity(6);

        util::add_quad(ctx, *self, param, &mut vertices, &mut indices);

        let vertices = VertexBuffer::immutable(&ctx.display, &vertices)?;
        let indices = IndexBuffer::immutable(&ctx.display, PrimitiveType::TrianglesList, &indices)?;

        let mut draw_parameters = DrawParameters::default();
        draw_parameters.backface_culling = BackfaceCullingMode::CullClockwise;
        draw_parameters.blend = Blend::alpha_blending();

        ctx.draw_calls.push(DrawCall::new(vertices, indices, self.id, draw_parameters));

        Ok(())
    }
}
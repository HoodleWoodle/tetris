use crate::ggwp::{
    GameResult, Context,
    graphics::{Drawable, Image, DrawParam, DrawCall},
    mint::Point2,
    util
};
use glium::{
    DrawParameters, BackfaceCullingMode, Blend,
    vertex::VertexBuffer,
    index::{PrimitiveType, IndexBuffer},
};

pub struct SpriteBatch {
    image: Image,

    draw_params: Vec<DrawParam>,
}

impl SpriteBatch {
    pub fn new(image: Image) -> Self {
        SpriteBatch {
            image,

            draw_params: vec![],
        }
    }

    pub fn add(&mut self, draw_param: DrawParam) {
        self.draw_params.push(draw_param);
    }

    pub fn clear(&mut self) {
        self.draw_params.clear();
    }
}

impl Drawable for SpriteBatch {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let mut vertices = Vec::with_capacity(self.draw_params.len() * 4);
        let mut indices = Vec::with_capacity(self.draw_params.len() * 6);

        for draw_param in self.draw_params.iter() {
            let dest = Point2::new(param.dest.x + draw_param.dest.x, param.dest.y + draw_param.dest.y);
            let color = param.color.combine(draw_param.color);
            let p = DrawParam::default()
                .color(color)
                .size(draw_param.size)
                .dest(dest)
                .src(draw_param.src);

            util::add_quad(ctx, self.image, p, &mut vertices, &mut indices);
        }
        
        let vertices = VertexBuffer::immutable(&ctx.display, &vertices)?;
        let indices = IndexBuffer::immutable(&ctx.display, PrimitiveType::TrianglesList, &indices)?;

        let mut draw_parameters = DrawParameters::default();
        draw_parameters.backface_culling = BackfaceCullingMode::CullClockwise;
        draw_parameters.blend = Blend::alpha_blending();

        ctx.draw_calls.push(DrawCall::new(vertices, indices, self.image.id, draw_parameters));

        Ok(())
    }
}
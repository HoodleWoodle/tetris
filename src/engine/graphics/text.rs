use crate::engine::{
    GameResult, Context,
    vec::{Vec2f, Vec2i, Vec2u},
    graphics::{
        self, DrawParam, Color, Image, Rect,
        spritebatch::SpriteBatch,
    },
    texture_packer::TexturePacker,
    util,
};
use std::{
    path::Path,
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
};
use freetype::face::{LoadFlag, Face};

const FONT_DEFAULT_SCALE: Scale = Scale { y: 16 };
const FONT_GLYPH_COUNT: usize = 128;
const FONT_DEFAULT_GLYPH: usize = 32;

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub struct Scale {
    pub y: u32,
}

impl Scale {
    pub fn uniform(s: f32) -> Self {
        Scale {
           y: s as u32,
        }
    }
}

pub struct FontData {
    face: Face,
    scaled: HashMap<Scale, Rc<RefCell<ScaledFontData>>>,
}

impl FontData {
    fn new(ctx: &mut Context, face: Face) -> usize {
        let font_data = FontData {
            face,
            scaled: HashMap::new(),
        };

        let id = ctx.fonts.len();
        ctx.fonts.push(Rc::new(RefCell::new(font_data)));

        id
    }

    fn scaled(&mut self, ctx: &mut Context, font: Font) -> GameResult<Rc<RefCell<ScaledFontData>>> {
        if !self.scaled.contains_key(&font.scale) {
            self.face.set_pixel_sizes(0, font.scale.y as u32)?;
            
            let mut packer = TexturePacker::new();
            let mut y_bearing_max = 0;
            let mut glyphs = Vec::with_capacity(FONT_GLYPH_COUNT);

            for c in 0..FONT_GLYPH_COUNT {
                self.face.load_char(c, LoadFlag::RENDER)?;
                let glyph = self.face.glyph();
                let bitmap = glyph.bitmap();
                let y_offset = glyph.bitmap_top();
                let size = Vec2i::new(bitmap.width(), bitmap.rows());

                // save result temporarily
                let temp = packer.pack(bitmap.buffer(), Vec2u::new(size.x as u32, size.y as u32));

                glyphs.push(Glyph {
                    advance: glyph.advance().x >> 6,
                    offset: Vec2i::new(glyph.bitmap_left(), y_offset),
                    size,
                    uv: Rect::new(temp.x as f32, temp.y as f32, 0.0, 0.0),
                });

                if y_bearing_max < y_offset {
                    y_bearing_max = y_offset;
                }
            }
            
            // adjust glyphs
            let packed_size = packer.size();
            let packed_data = packer.data();

            for glyph in glyphs.iter_mut() {
                // update y offset
                glyph.offset.y = y_bearing_max - glyph.offset.y;

                // update uv coordinates
                let origin = Vec2f::new(glyph.uv.x, glyph.uv.y);
                glyph.uv.x = origin.x / (packed_size.x as f32);
                glyph.uv.y = origin.y / (packed_size.y as f32);

                glyph.uv.w = (glyph.size.x as f32) / (packed_size.x as f32);
                glyph.uv.h = (glyph.size.y as f32) / (packed_size.y as f32);
            }

            // convert bitmap to RGBA
            let pixels = (packed_size.x * packed_size.y) as usize;
            let mut data = Vec::with_capacity(pixels * 4);
            for i in 0..pixels {
                data.push(0xFF);            // R
                data.push(0xFF);            // G
                data.push(0xFF);            // B
                data.push(packed_data[i]);  // A
            }
            let image = Image::from_raw(ctx, (packed_size.x, packed_size.y), data)?;

            let scaled_data = ScaledFontData {
                image,
                glyphs,
            };
            self.scaled.insert(font.scale, Rc::new(RefCell::new(scaled_data)));
        }

        Ok(Rc::clone(self.scaled.get(&font.scale).unwrap()))
    }
}

struct ScaledFontData {
    image: Image,
    glyphs: Vec<Glyph>,
}

struct Glyph {
    advance: i32,
    offset: Vec2i,
    size: Vec2i,
    uv: Rect,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Font {
    id: usize,
    scale: Scale,
}

impl Font {
    pub fn new<P>(ctx: &mut Context, path: P) -> GameResult<Font>
    where
        P: AsRef<Path>
    {
        let path = util::get_final_path(path)?;
        let face = ctx.ft_lib.new_face(path, 0)?;

        let id = FontData::new(ctx, face);
        let font = Font {
            id,
            scale: FONT_DEFAULT_SCALE,
        };

        Ok(font)
    }

    fn scale(&self, scale: Scale) -> Font {
        Font {
            id: self.id,
            scale,
        }
    }

    fn data(&self, ctx: &mut Context) -> GameResult<Rc<RefCell<ScaledFontData>>> {
        Rc::clone(&ctx.fonts[self.id]).borrow_mut().scaled(ctx, self.clone())
    }
}

#[derive(Clone)]
pub struct TextFragment {
    text: String,
}

impl<'a> From<&'a str> for TextFragment {
    fn from(text: &'a str) -> TextFragment {
        TextFragment {
            text: text.to_owned(),
        }
    }
}

impl From<String> for TextFragment {
    fn from(text: String) -> TextFragment {
        TextFragment {
            text
        }
    }
}

#[derive(Clone)]
pub struct Text {
    font: Option<Font>,
    fragment: TextFragment,
}

impl Text {
    pub fn new<F>(fragment: F) -> Self
    where
        F: Into<TextFragment>
    {
        Text {
            font: None,
            fragment: fragment.into(),
        }
    }

    pub fn set_font(&mut self, font: Font, font_scale: Scale) -> &mut Text {
        self.font = Some(font.scale(font_scale));
        self
    }
    
    fn size(&self, ctx: &mut Context) -> Vec2u {
        match self.font {
            Some(font) => {
                let data = font.data(ctx).unwrap();
                let borrow = data.borrow();

                let mut w = 0;
                let mut h = 0;

                for c in self.fragment.text.chars() {
                    let idx = c as usize;
                    let g = if idx < FONT_GLYPH_COUNT {
                        &borrow.glyphs[c as usize]
                    } else {
                        &borrow.glyphs[FONT_DEFAULT_GLYPH]
                    };

                    w += g.advance;

                    let h_new = g.offset.y + g.size.y;
                    if h < h_new {
                        h = h_new;
                    }
                }

                Vec2u::new(w as u32, h as u32)
            },
            None => Vec2u::new(0, 0),
        }
    }

    pub fn width(&self, ctx: &mut Context) -> u32 {
        self.size(ctx).x
    }

    pub fn height(&self, ctx: &mut Context) -> u32 {
        self.size(ctx).y
    }
}

#[derive(Clone)]
pub struct QueuedText {
    text: Text,
    relative_dest: Vec2f,
    color: Color,
}

impl QueuedText {
    fn new(text: &Text, relative_dest: Vec2f, color: Color) -> QueuedText {
        QueuedText {
            text: text.clone(),
            relative_dest,
            color
        }
    }
}

pub enum BlendMode {
}

pub enum FilterMode {
    Linear
}

pub fn queue_text<P>(ctx: &mut Context, batch: &Text, relative_dest: P, color: Option<Color>)
where
    P: Into<Vec2f>,
{
    let relative_dest = relative_dest.into();
    let color = color.unwrap_or(graphics::WHITE);

    ctx.texts.push(QueuedText::new(batch, relative_dest, color));
}

pub fn draw_queued_text<D>(ctx: &mut Context, param: D, _: Option<BlendMode>, _: FilterMode) -> GameResult<()>
where
    D: Into<DrawParam>
{
    let mut batches: HashMap<Font, SpriteBatch> = HashMap::new();

    let texts = ctx.texts.clone();
    for text in texts {
        let font = text.text.font.unwrap();
        let data = Rc::clone(&font.data(ctx)?);
        let borrow = data.borrow();

        if !batches.contains_key(&font) {
            batches.insert(font, SpriteBatch::new(borrow.image));
        }
        let batch = batches.get_mut(&font).unwrap();

        // xD: text.text.fragment.text

        let mut cursor = text.relative_dest;
        for c in text.text.fragment.text.chars() {
            let idx = c as usize;
            let g = if idx < FONT_GLYPH_COUNT {
                &borrow.glyphs[c as usize]
            } else {
                &borrow.glyphs[FONT_DEFAULT_GLYPH]
            };

            let mut dest = cursor;
            dest.x += g.offset.x as f32;
            dest.y += g.offset.y as f32;

            let param = DrawParam::default()
                .color(text.color)
                .dest(dest)
                .src(g.uv);
            batch.add(param);
            
            cursor.x += g.advance as f32;
        }
    }

    ctx.texts.clear();
    let param = param.into();
    for batch in batches.values_mut() {
        graphics::draw(ctx, batch, param.clone())?;
    }
    
    Ok(())
}
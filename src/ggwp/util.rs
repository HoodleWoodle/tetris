use crate::ggwp::{
    GameResult, GameError, Context,
    graphics::{Vtx, DrawParam, Image},
};
use image::{self, RgbaImage};
use std::{
    path::{Path, PathBuf},
    env,
};
use glium::glutin::dpi::LogicalSize;

pub fn add_quad(ctx: &mut Context, image: Image, param: DrawParam, vertices: &mut Vec<Vtx>, indices: &mut Vec<u16>) {
    let (mut tw, mut th) = image.size(ctx);
    tw *= param.src.w;
    th *= param.src.h;

    if let Some(size) = param.size {
        tw = size.x;
        th = size.y;
    }
    
    let scale_factor = ctx.display.gl_window().window().scale_factor();
    let window_size: LogicalSize<u32> = ctx.display.gl_window().window().inner_size().to_logical(scale_factor);
    let (ww, wh) = (window_size.width as f32, window_size.height as f32);

    let dest_left = param.dest.x / ww * 2.0 - 1.0;
    let dest_right = (param.dest.x + tw) / ww * 2.0 - 1.0;
    let dest_top = (wh - param.dest.y) / wh * 2.0 - 1.0;
    let dest_bottom = ((wh - param.dest.y) - th) / wh * 2.0 - 1.0;

    let src_left = param.src.x;
    let src_right = src_left + param.src.w;
    let src_top = param.src.y;
    let src_bottom = src_top + param.src.h;

    let idx = vertices.len() as u16;

    vertices.push(Vtx::new([dest_left, dest_bottom], [src_left, src_bottom], param.color.into()));
    vertices.push(Vtx::new([dest_right, dest_bottom], [src_right, src_bottom], param.color.into()));
    vertices.push(Vtx::new([dest_right, dest_top], [src_right, src_top], param.color.into()));
    vertices.push(Vtx::new([dest_left, dest_top], [src_left, src_top], param.color.into()));

    indices.extend_from_slice(&[idx + 0, idx + 1, idx + 2, idx + 2, idx + 3, idx + 0]);
}

pub fn image<P>(path: P) -> GameResult<RgbaImage>
where
    P: AsRef<Path>
{
    let path = get_final_path(path)?;
    let img = image::open(path)?;

    Ok(img.into_rgba())
}

pub fn get_final_path<P>(path: P) -> GameResult<PathBuf>
where
    P: AsRef<Path>
{
    let pathbuf;
    
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        pathbuf = PathBuf::from(manifest_dir + "/resources" + path.as_ref().to_str().unwrap());
    } else {
        pathbuf = PathBuf::from("resources".to_owned() + path.as_ref().to_str().unwrap());
    }
    
    if pathbuf.exists() {
        Ok(pathbuf)
    } else {
        Err(GameError::FileNotFound(pathbuf))
    }
}
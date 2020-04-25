use crate::ggwp::Context;

use std::{
    thread,
    time::Duration,
};

pub fn ticks(ctx: &Context) -> usize {
    ctx.ticks
}

pub fn frames(ctx: &Context) -> usize {
    ctx.frames
}

pub fn fps(ctx: &Context) -> usize {
    ctx.fps
}

pub fn ups(ctx: &Context) -> usize {
    ctx.ups
}

pub fn yield_now() {
    thread::yield_now();
}

pub fn check_update_time(ctx: &mut Context, target_fps: u32) -> bool {
    let target_dt = Duration::from_secs(1) / target_fps;

    if ctx.residual_update_dt > target_dt {
        ctx.ticks += 1;
        ctx.residual_update_dt -= target_dt;
        true
    } else {
        false
    }
}
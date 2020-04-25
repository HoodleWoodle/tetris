use crate::ggwp::{GameResult, GameError, Context};

use glium::{
    Surface,
    glutin::{
        event::{Event, WindowEvent, KeyboardInput, ElementState},
        event_loop::{EventLoop, EventLoopWindowTarget, ControlFlow},
        platform::desktop::EventLoopExtDesktop,
    },
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
};
use std::time::{Instant, Duration};

pub trait EventHandler {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool);
}

pub use glium::glutin::event::VirtualKeyCode as KeyCode;

pub enum KeyMods {
    NotImplemented
}

fn handle<S>(ctx: &mut Context, handler: &mut S, event: Event<'_, ()>, _: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow) -> GameResult<()>
where
    S: EventHandler
{
    *control_flow = ControlFlow::Poll;

    match event {
        Event::WindowEvent { event, .. } => {
            match event {
                WindowEvent::CloseRequested { .. } => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::KeyboardInput { input: KeyboardInput { state, virtual_keycode: Some(keycode), .. }, is_synthetic: false, .. } => {
                    let index = keycode as usize;

                    if state == ElementState::Pressed {
                        let repeat = ctx.key_states[index];
                        handler.key_down_event(ctx, keycode, KeyMods::NotImplemented, repeat);
                    }

                    ctx.key_states[index] = state == ElementState::Pressed;
                },

                _ => ()
            }
        },
        Event::MainEventsCleared => {
            // Application update code.

            let now = Instant::now();
            let duration_since = now.duration_since(ctx.last_time);
            ctx.duration += duration_since;
            ctx.residual_update_dt += duration_since;
            ctx.last_time = now;

            if ctx.duration.as_secs() >= 1 {
                ctx.duration -= Duration::from_secs(1);
                
                ctx.ups = ctx.ticks - ctx.last_ticks;
                ctx.last_ticks = ctx.ticks;
                
                ctx.fps = ctx.frames - ctx.last_frames;
                ctx.last_frames = ctx.frames;
            }

            handler.update(ctx)?;

            // Queue a RedrawRequested event.
            ctx.display.gl_window().window().request_redraw();
        },
        Event::RedrawRequested(_) => {
            // Redraw the application.
            //
            // It's preferrable to render in this event rather than in MainEventsCleared, since
            // rendering in here allows the program to gracefully handle redraws requested
            // by the OS.

            handler.draw(ctx)?;

            let mut frame = ctx.display.draw();
            frame.clear_color(0.0, 0.0, 0.0, 0.0);

            for draw_call in ctx.draw_calls.iter() {
                let texture = &ctx.textures[draw_call.texture_id];

                let uniforms = uniform! {
                    u_texture: texture.sampled()
                                .magnify_filter(MagnifySamplerFilter::Nearest)
                                .minify_filter(MinifySamplerFilter::Nearest)
                };

                frame.draw(&draw_call.vertices, &draw_call.indices, &ctx.program, &uniforms, &draw_call.draw_parameters)?;
            }
            ctx.draw_calls.clear();

            frame.finish()?;
            
            ctx.frames += 1;
        },
        _ => ()
    }

    Ok(())
}
    
pub fn run<S>(ctx: &mut Context, event_loop: &mut EventLoop<()>, handler: &mut S) -> GameResult<()>
where
    S: EventHandler
{
    let mut result = Ok(());

    event_loop.run_return(|event, target, control_flow| {
        if let Err(err) = handle(ctx, handler, event, target, control_flow) {
            result = Err(GameError::from(err));
            *control_flow = ControlFlow::Exit;
        }
    });

    result
}
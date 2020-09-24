use crate::engine::{
    Context,
    event::KeyCode
};

pub fn is_key_pressed(ctx: &Context, key: KeyCode) -> bool {
    ctx.key_states[key as usize]
}
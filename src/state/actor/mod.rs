use ggez::Context;

pub mod player;

#[derive(Copy, Clone)]
pub enum Action {
    RotateLeft,
    RotateRight,
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    Drop,
}

pub trait Actor {
    fn is_auto_drop(&self) -> bool {
        true
    }

    fn check(&mut self, ctx: &mut Context, action: Action) -> bool;
    fn push(&mut self, _action: Action) {
    }

    fn update(&mut self, ctx: &mut Context);
}
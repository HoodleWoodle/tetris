use crate::engine::{
    event::KeyCode,
    input::keyboard,
    Context,
};

use super::{Actor, Action};

pub struct Player {
    was_rcontrol_pressed: bool,
    was_y_pressed: bool,
    was_z_pressed: bool,
    was_up_pressed: bool,
    was_x_pressed: bool,
    was_space_pressed: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            was_rcontrol_pressed: false,
            was_y_pressed: false,
            was_z_pressed: false,
            was_up_pressed: false,
            was_x_pressed: false,
            was_space_pressed: false,
        }
    }

    fn is_valid(&mut self, keycode: KeyCode) -> bool {
        match keycode {
            KeyCode::RControl => !self.was_rcontrol_pressed,
            KeyCode::Y => !self.was_y_pressed,
            KeyCode::Z => !self.was_z_pressed,
            KeyCode::Up => !self.was_up_pressed,
            KeyCode::X => !self.was_x_pressed,
            KeyCode::Space => !self.was_space_pressed,
            _ => true,
        }
    }

    fn check_keys(&mut self, ctx: &mut Context, keycodes: &[KeyCode]) -> bool {
        for keycode in keycodes {
            if keyboard::is_key_pressed(ctx, *keycode) {
                if self.is_valid(*keycode) {
                    return true;
                }
            }
        }

        false
    }
}

impl Actor for Player {
    fn check(&mut self, ctx: &mut Context, action: Action) -> bool {
        match action {
            Action::RotateLeft => self.check_keys(ctx, &[KeyCode::RControl, KeyCode::Y, KeyCode::Z]),
            Action::RotateRight => self.check_keys(ctx, &[KeyCode::Up, KeyCode::X]),
            Action::MoveLeft => self.check_keys(ctx, &[KeyCode::Left]),
            Action::MoveRight => self.check_keys(ctx, &[KeyCode::Right]),
            Action::SoftDrop => self.check_keys(ctx, &[KeyCode::Down]),
            Action::HardDrop => self.check_keys(ctx, &[KeyCode::Space]),
            Action::Drop => false,
        }
    }

    fn update(&mut self, ctx: &mut Context) {
        self.was_rcontrol_pressed = keyboard::is_key_pressed(ctx, KeyCode::RControl);
        self.was_y_pressed = keyboard::is_key_pressed(ctx, KeyCode::Y);
        self.was_z_pressed = keyboard::is_key_pressed(ctx, KeyCode::Z);
        self.was_up_pressed = keyboard::is_key_pressed(ctx, KeyCode::Up);
        self.was_x_pressed = keyboard::is_key_pressed(ctx, KeyCode::X);
        self.was_space_pressed = keyboard::is_key_pressed(ctx, KeyCode::Space);
    }
}
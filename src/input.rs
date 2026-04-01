use macroquad::prelude::*;
use crate::{game::{Game, Direction}, Window};

pub fn handle_input(game: &mut Game, window: &mut Window) {
    if is_key_pressed(KeyCode::F11) {
        window.is_fullscreen = !window.is_fullscreen;
        if window.is_fullscreen {
            set_fullscreen(true);
        } else {
            set_fullscreen(false);
        }
    }

    if game.is_active() {
        if is_key_pressed(KeyCode::Space) {
            game.place();
        }

        if is_key_pressed(KeyCode::Up) {
            game.rotate_active(true);
        }

        if is_key_pressed(KeyCode::Z) {
            game.rotate_active(false);
        }

        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::RightShift) {
            game.hold();
        } 

        let now = macroquad::time::get_time();

        // Horizontal movement
        let x_dir = if is_key_down(KeyCode::Left) {
            Some(Direction::Left)
        } else if is_key_down(KeyCode::Right) {
            Some(Direction::Right)
        } else {
            None
        };

        let just_began: bool;
        if x_dir != game.x_move_dir {
            game.x_move_dir = x_dir;
            game.x_move_begin = macroquad::time::get_time();
            just_began = true;
        } else {
            just_began = false;
        }

        // if the movement just began, there should be a delay before the piece can move quickly. This allows for more precise movement.
        if let Some(dir) = x_dir {
            if just_began {
                game.move_active(dir);
            } else {
                if now - game.x_move_begin >= 0.16 {
                    if now - game.x_last_move_update >= 0.033 {
                        game.move_active(dir);
                        game.x_last_move_update = now;
                    }
                }
            }
        }

        // Vertical movement
        if is_key_down(KeyCode::Down) {
            if now - game.y_last_move_update >= 0.05 {
                game.move_active(Direction::Down);
                game.y_last_move_update = now;
            }
        }
    } else {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            game.reset_game();
            game.drop_next();
        }
    }
}
// TETRIS

use macroquad::prelude::*;


mod draw;
mod game;
mod input;
mod rotation;
use draw::draw_game;
use game::Game;
use input::handle_input;

struct Window {
    pub is_fullscreen: bool
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_height: 600,
        window_width: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    rand::srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    #[cfg(target_arch = "wasm32")]
    rand::srand(quad_timestamp::timestamp_utc_ms().unwrap() as u64);

    let mut game = Game::new();
    let mut window = Window {
        is_fullscreen: false
    };

    game.drop_next();

    loop {
        handle_input(&mut game, &mut window);

        if game.is_active() {
            game.update_gravity();
        }

        draw_game(&game);
        next_frame().await;
    }
}
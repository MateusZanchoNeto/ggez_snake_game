mod app;
mod ate;
mod constants;
mod direction;
mod food;
mod game_state;
mod grid_position;
mod segment;
mod snake;

use crate::app::App;
use crate::game_state::GameState;

fn main() {
    let app = App::new();

    app.run(GameState::new())
}

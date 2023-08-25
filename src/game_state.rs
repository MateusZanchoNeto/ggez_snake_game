use crate::ate::Ate;
use crate::constants::*;
use crate::direction::Direction;
use crate::food::Food;
use crate::grid_position::GridPosition;
use crate::snake::Snake;

use ggez::{
    event::{EventHandler, KeyCode},
    graphics, Context, GameResult,
};
use std::time::{Duration, Instant};

pub struct GameState {
    snake: Snake,
    food: Food,
    gameover: bool,
    last_update: Instant,
}

impl GameState {
    pub fn new() -> Self {
        let snake_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        let food_pos = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);
        Self {
            snake: Snake::new(snake_pos),
            food: Food::new(food_pos),
            gameover: false,
            last_update: Instant::now(),
        }
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        if (Instant::now() - self.last_update < Duration::from_millis(MILLIS_PER_UPDATE))
            || self.gameover
        {
            return Ok(());
        }

        self.snake.update(&self.food);

        if let Some(ate) = self.snake.get_ate() {
            match ate {
                Ate::Food => {
                    let new_food_position = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);
                    self.food.position = new_food_position;
                }

                Ate::Itself => {
                    self.gameover = true;
                }
            }
        }
        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, [0.0, 1.0, 0.0, 1.0].into());

        self.snake.draw(context)?;
        self.food.draw(context)?;

        graphics::present(context)?;

        ggez::timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        if let Some(direction) = Direction::from_keycode(keycode) {
            if direction.inverse() != self.snake.get_last_update_direction() {
                self.snake.set_direction(direction);
            }
        }
    }
}

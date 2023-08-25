use crate::{constants::*, direction::Direction};
use rand::Rng;

trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
where
    T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo(&self, n: T) -> T {
        (self.clone() % n.clone() + n.clone()) % n
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn random(max_x: i16, max_y: i16) -> Self {
        let mut thread_rng = rand::thread_rng();
        (
            thread_rng.gen_range(0..max_x),
            thread_rng.gen_range(0..max_y),
        )
            .into()
    }

    pub fn new_from_move(position: Self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(position.x, (position.y - 1).modulo(GRID_SIZE.1)),
            Direction::Down => Self::new(position.x, (position.y + 1).modulo(GRID_SIZE.1)),
            Direction::Left => Self::new((position.x - 1).modulo(GRID_SIZE.0), position.y),
            Direction::Right => Self::new((position.x + 1).modulo(GRID_SIZE.0), position.y),
        }
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }
}

impl From<GridPosition> for ggez::graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        ggez::graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

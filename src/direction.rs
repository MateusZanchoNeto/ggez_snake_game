use ggez::event::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::Up => Some(Self::Up),
            KeyCode::Down => Some(Self::Down),
            KeyCode::Left => Some(Self::Left),
            KeyCode::Right => Some(Self::Right),
            _ => None,
        }
    }
}

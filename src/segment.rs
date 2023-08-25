use crate::grid_position::GridPosition;

#[derive(Clone, Copy, Debug)]
pub struct Segment {
    position: GridPosition,
}

impl Segment {
    pub fn new(position: GridPosition) -> Self {
        Self { position }
    }

    pub fn get_position(&self) -> GridPosition {
        self.position
    }
}

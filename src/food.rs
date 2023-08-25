use crate::grid_position::GridPosition;
use ggez::{graphics, Context, GameResult};

pub struct Food {
    pub position: GridPosition,
}

impl Food {
    pub fn new(position: GridPosition) -> Self {
        Self { position }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        let mesh = graphics::MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                self.position.into(),
                graphics::Color::new(0.0, 0.0, 1.0, 1.0),
            )?
            .build(context)?;

        graphics::draw(context, &mesh, graphics::DrawParam::default())
    }
}

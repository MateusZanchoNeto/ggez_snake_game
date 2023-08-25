use crate::ate::Ate;
use crate::direction::Direction;
use crate::food::Food;
use crate::grid_position::GridPosition;
use crate::segment::Segment;

use ggez::{graphics, Context, GameResult};
use std::collections::LinkedList;

pub struct Snake {
    head: Segment,
    direction: Direction,
    body: LinkedList<Segment>,
    ate: Option<Ate>,
    last_update_direction: Direction,
}

impl Snake {
    pub fn new(position: GridPosition) -> Self {
        let mut body = LinkedList::new();
        body.push_back(Segment::new(
            (position.get_x() - 1, position.get_y()).into(),
        ));
        Self {
            head: Segment::new(position),
            direction: Direction::Right,
            last_update_direction: Direction::Right,
            body,
            ate: None,
        }
    }

    fn eats(&self, food: &Food) -> bool {
        self.head.get_position() == food.position
    }

    fn eats_self(&self) -> bool {
        for segment in self.body.iter() {
            if self.head.get_position() == segment.get_position() {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, food: &Food) {
        let new_head_position =
            GridPosition::new_from_move(self.head.get_position(), self.direction);
        let new_head = Segment::new(new_head_position);

        self.body.push_front(self.head);
        self.head = new_head;

        if self.eats_self() {
            self.ate = Some(Ate::Itself);
        } else if self.eats(food) {
            self.ate = Some(Ate::Food);
        } else {
            self.ate = None
        }

        if self.ate.is_none() {
            self.body.pop_back();
        }

        self.last_update_direction = self.direction;
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        for segment in self.body.iter() {
            let mesh = graphics::MeshBuilder::new()
                .rectangle(
                    graphics::DrawMode::fill(),
                    segment.get_position().into(),
                    graphics::Color::new(1.0, 0.5, 0.0, 1.0),
                )?
                .build(context)?;
            graphics::draw(context, &mesh, graphics::DrawParam::default())?;
        }

        let mesh = graphics::MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                self.head.get_position().into(),
                graphics::Color::new(1.0, 0.0, 0.0, 1.0),
            )?
            .build(context)?;
        graphics::draw(context, &mesh, graphics::DrawParam::default())
    }

    pub fn get_ate(&self) -> Option<Ate> {
        self.ate
    }

    pub fn get_last_update_direction(&self) -> Direction {
        self.last_update_direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }
}

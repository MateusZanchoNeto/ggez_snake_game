use crate::constants::*;
use crate::game_state::GameState;
use ggez::{
    conf::{WindowMode, WindowSetup},
    event::EventLoop,
    Context, ContextBuilder,
};
pub struct App {
    context: Context,
    event_loop: EventLoop<()>,
}

impl App {
    pub fn new() -> Self {
        let (context, event_loop) = ContextBuilder::new("snake", "Mateus Zancho Neto")
            .window_setup(WindowSetup::default().title("Snake"))
            .window_mode(
                WindowMode::default().dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
            )
            .build()
            .expect("Failed to build ggez context");

        Self {
            context,
            event_loop,
        }
    }

    pub fn run(self, state: GameState) {
        ggez::event::run(self.context, self.event_loop, state)
    }
}

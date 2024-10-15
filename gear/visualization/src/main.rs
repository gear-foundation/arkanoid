use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize};
use ggez::timer;


fn load_game_steps() -> Vec<GameStep> {
    let file = File::open("game_steps.json").expect("Unable to open file");
    let reader = BufReader::new(file);
    let game_steps: Vec<GameStep> = serde_json::from_reader(reader).expect("Unable to parse file");
    game_steps
}

#[derive(Deserialize)]
struct GameStep {
    ball_x: f32,
    ball_y: f32,
    ball_velocity_x: f32,
    ball_velocity_y: f32,
    paddle_x: f32,
    paddle_y: f32,
    block_hit: Option<(f32, f32)>,
}

struct Block {
    rect_x1: f32,
    rect_y1: f32,
    rect_x2: f32,
    rect_y2: f32,
}

impl Block {
    fn new(x1: f32, y1: f32, width: f32, height: f32) -> Self {
        Block {
            rect_x1: x1,
            rect_y1: y1,
            rect_x2: x1 + width,
            rect_y2: y1 + height,
        }
    }
}

struct MainState {
    ball_x: f32,
    ball_y: f32,
    ball_radius: f32,
    paddle_x: f32,
    paddle_y: f32,
    paddle_width: f32,
    paddle_height: f32,
    blocks: Vec<Block>,
    events: Vec<GameStep>,
    current_event: usize,
    time_since_last_update: f32, 
}

impl MainState {
    fn new(events: Vec<GameStep>) -> Self {
        // Initialize blocks, paddle, and ball
        let mut blocks = Vec::new();
        let block_width = 30;
        let block_height = 30;
        let rows = 7;
        let cols = 30;

        for row in 0..rows {
            for col in 0..cols {
                let x = col * (block_width + 5);
                let y = row * (block_height + 5);
                blocks.push(Block::new(
                    x as f32,
                    y as f32,
                    block_width as f32,
                    block_height as f32,
                ));
            }
        }

        MainState {
            ball_x: 400.0,
            ball_y: 300.0,
            ball_radius: 15.0,
            paddle_x: 375.0,
            paddle_y: 550.0,
            paddle_width: 400.0,
            paddle_height: 10.0,
            blocks,
            events,
            current_event: 0,
            time_since_last_update: 0.0, 
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta_time = timer::delta(ctx).as_secs_f32();
        self.time_since_last_update += delta_time;

        if self.time_since_last_update >= 0.0 {
            self.time_since_last_update = 0.0;
            if self.current_event < self.events.len() {
                let game_step = &self.events[self.current_event];
                self.ball_x = game_step.ball_x;
                self.ball_y = game_step.ball_y;
                self.paddle_x = game_step.paddle_x;
                self.paddle_y = game_step.paddle_y;
                // If a block was hit, remove it
                if let Some((hit_x, hit_y)) = game_step.block_hit {
                    self.blocks
                        .retain(|block| block.rect_x1 != hit_x || block.rect_y1 != hit_y);
                }
                // self.apply_event(event);
                self.current_event += 1; // Move to the next event
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        // Draw ball
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2 {
                x: self.ball_x,
                y: self.ball_y,
            },
            self.ball_radius,
            2.0,
            Color::BLUE,
        )?;
        graphics::draw(ctx, &circle, (Point2 { x: 0.0, y: 0.0 },))?;

        // Draw paddle
        let paddle_rect = graphics::Rect::new(
            self.paddle_x,
            self.paddle_y,
            self.paddle_width,
            self.paddle_height,
        );
        let paddle = Mesh::new_rectangle(ctx, DrawMode::fill(), paddle_rect, Color::RED)?;
        graphics::draw(ctx, &paddle, (Point2 { x: 0.0, y: 0.0 },))?;

        // Draw blocks
        for block in &self.blocks {
            let rect = graphics::Rect::new(
                block.rect_x1,
                block.rect_y1,
                block.rect_x2 - block.rect_x1,
                block.rect_y2 - block.rect_y1,
            );
            let rectangle = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?;
            graphics::draw(ctx, &rectangle, (Point2 { x: 0.0, y: 0.0 },))?;
        }

        graphics::present(ctx)
    }
}


fn main() -> GameResult {
    // Load game steps from the JSON file
    let game_steps = load_game_steps();

    let (ctx, event_loop) = ggez::ContextBuilder::new("game_visualization", "Author")
        .build()
        .expect("Failed to build ggez context");

    let state = MainState::new(game_steps);
    event::run(ctx, event_loop, state)
}
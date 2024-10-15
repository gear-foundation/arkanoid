use rust_decimal::Decimal;
use sails_rs::{gstd::debug, prelude::*};

use crate::Events;

pub const BLOCK_WIDTH: i16 = 30;
pub const BLOCK_HEIGHT: i16 = 30;

#[derive(Default, Encode, Decode, TypeInfo, Clone)]
pub struct Ball {
    x: i16,
    y: i16,
    radius: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            x: 400,
            y: 300,
            radius: 15,
            velocity_x: 3,
            velocity_y: 3,
        }
    }
}

#[derive(Default)]
pub struct Block {
    rect_x1: i16,
    rect_y1: i16,
    rect_x2: i16,
    rect_y2: i16,
}

impl Block {
    pub fn new(x1: i16, y1: i16) -> Self {
        Block {
            rect_x1: x1,
            rect_y1: y1,
            rect_x2: x1 + BLOCK_WIDTH,
            rect_y2: y1 + BLOCK_HEIGHT,
        }
    }
}

#[derive(Default, Encode, Decode, TypeInfo, Clone)]
pub struct Paddle {
    x: i16,
    y: i16,
    width: i16,
    speed: i16,
    direction: i16,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle {
            x: 375,
            y: 550,
            width: 400,
            speed: 5,
            direction: 1,
        }
    }

    // Updates the paddle's position and reverses direction at screen boundaries
    pub fn update_position(&mut self) {
        self.x += self.speed * self.direction;

        // Reverse direction if the paddle reaches the screen edges
        if self.x <= 0 || self.x + self.width >= 800 {
            // Change direction
            self.direction = -self.direction;
        }
    }
}

#[derive(Default)]
pub struct Game {
    ball: Ball,
    blocks: Vec<Block>,
    paddle: Paddle,
}

impl Game {
    pub fn new() -> Self {
        let mut blocks = Vec::new();

        let rows = 7;
        let cols = 30;

        for row in 0..rows {
            for col in 0..cols {
                let x = col * 35;
                let y = row * 35;
                blocks.push(Block::new(x, y));
            }
        }

        let paddle = Paddle::new();

        Game {
            ball: Ball::new(),
            blocks,
            paddle,
        }
    }

    pub fn update_game(&mut self) -> Events {
        // Move the ball
        self.ball.x += self.ball.velocity_x;
        self.ball.y += self.ball.velocity_y;

        // Move the paddle based on its direction and speed
        self.paddle.update_position();

        // Variables to capture the event information
        let mut wall_collision = None;
        let mut paddle_collision = false;
        let mut block_hit = None;

        // Check if the ball collides with the screen edges and reverse its direction if needed
        if self.ball.x - self.ball.radius <= 0 || self.ball.x + self.ball.radius >= 800 {
            self.ball.velocity_x = -self.ball.velocity_x;
            wall_collision = Some("Vertical".to_string());
        }
        if self.ball.y - self.ball.radius <= 0 || self.ball.y + self.ball.radius >= 600 {
            self.ball.velocity_y = -self.ball.velocity_y;
            wall_collision = Some("Horizontal".to_string());
        }

        // Check if the ball hits the paddle
        if self.ball.y + self.ball.radius >= self.paddle.y
            && self.ball.x >= self.paddle.x
            && self.ball.x <= self.paddle.x + self.paddle.width
        {
            self.ball.velocity_y = -self.ball.velocity_y;
            // let paddle_center = self.paddle.x + (self.paddle.width / 2);
            //  let distance_from_center = self.ball.x - paddle_center;
            //  self.ball.x += distance_from_center * Decimal::new(5, 2);
            paddle_collision = true;
        }

        // Check if the ball collides with any blocks and stop once a collision is detected
        for block in self.blocks.iter_mut() {
            if let Some((collision_x, collision_y)) = check_circle_rectangle_collision(
                self.ball.x,
                self.ball.y,
                self.ball.radius,
                block.rect_x1,
                block.rect_y1,
                block.rect_x2,
                block.rect_y2,
            ) {
                if collision_x {
                    self.ball.velocity_x = -self.ball.velocity_x;
                }
                if collision_y {
                    self.ball.velocity_y = -self.ball.velocity_y;
                }
                // Remove the block from the game
                block_hit = Some((block.rect_x1, block.rect_y1));
                break;
            }
        }

        // Filter out hit blocks from the game
        self.blocks.retain(|block| {
            !block_hit.is_some()
                || block.rect_x1 != block_hit.unwrap().0
                || block.rect_y1 != block_hit.unwrap().1
        });

        Events::GameStep {
            ball: self.ball.clone(),
            paddle: self.paddle.clone(),
            wall_collision,
            paddle_collision,
            block_hit,
        }
    }
}

// Function to check collision between the ball (circle) and blocks (rectangles)
fn check_circle_rectangle_collision(
    circle_x: i16,
    circle_y: i16,
    radius: i16,
    rect_x1: i16,
    rect_y1: i16,
    rect_x2: i16,
    rect_y2: i16,
) -> Option<(bool, bool)> {
    let nearest_x = rect_x1.max(circle_x.min(rect_x2));
    let nearest_y = rect_y1.max(circle_y.min(rect_y2));

    let distance_x = (circle_x - nearest_x) as i64;
    let distance_y = (circle_y - nearest_y) as i64;
    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    let radius_squared = (radius * radius) as i64;

    if distance_squared <= radius_squared {
        let collision_x = nearest_x == rect_x1 || nearest_x == rect_x2;
        let collision_y = nearest_y == rect_y1 || nearest_y == rect_y2;
        Some((collision_x, collision_y))
    } else {
        None
    }
}

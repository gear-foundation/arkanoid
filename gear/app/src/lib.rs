#![no_std]

use sails_rs::{gstd::msg, prelude::*};
mod game;
use game::{Ball, Game, Paddle};
static mut GAME: Option<Game> = None;
struct VaraArkanoidService(());

impl VaraArkanoidService {
    pub fn init() -> Self {
        unsafe { GAME = Some(Game::new()) }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Game {
        unsafe { GAME.as_mut().expect("GAME is not initialized") }
    }
    pub fn get(&self) -> &'static Game {
        unsafe { GAME.as_ref().expect("GAME is not initialized") }
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Events {
    GameStep {
        ball: Ball,
        paddle: Paddle,
    },
}

#[sails_rs::service(events = Events)]
impl VaraArkanoidService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn simulate_game(&mut self, num_steps: u32) {
        for _ in 0..num_steps {
            let event = self.get_mut().update_game();
            self.notify_on(event).expect("");
        }
    }
}

pub struct VaraArkanoidProgram(());

#[sails_rs::program]
impl VaraArkanoidProgram {
    // Program's constructor
    pub fn new() -> Self {
        VaraArkanoidService::init();
        Self(())
    }

    // Exposed service
    pub fn vara_arkanoid(&self) -> VaraArkanoidService {
        VaraArkanoidService::new()
    }
}

use futures::stream::StreamExt;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use sails_rs::{
    calls::*,
    events::Listener,
    gtest::{calls::*, System},
};
use vara_arkanoid_client::traits::*;
use vara_arkanoid_client::vara_arkanoid::events::{self, VaraArkanoidEvents};
use serde::{Serialize};
use std::fs::File;
use std::io::Write;
use serde_json;


const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn simulate_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(vara_arkanoid::WASM_BINARY);

    let program_factory = vara_arkanoid_client::VaraArkanoidFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = vara_arkanoid_client::VaraArkanoid::new(remoting.clone());

    let mut listener = events::listener(remoting);

    let mut events = listener.listen().await.unwrap();

    let steps = 1000;
    service_client
        .simulate_game(steps)
        .send_recv(program_id)
        .await
        .unwrap();

    let mut game_steps = Vec::new();
    for i in 0..steps {
        let event = events.next().await.unwrap();
        let VaraArkanoidEvents::GameStep {
            ball,
            paddle,
            wall_collision,
            paddle_collision,
            block_hit,
        } = event.1;
        let block_hit = if let Some((x, y)) = block_hit {
            Some((x as f32, y as f32))
        } else {
            None
        };
        game_steps.push(GameStep {
            ball_x: ball.x as f32,
            ball_y: ball.y as f32,
            ball_velocity_x: ball.velocity_x as f32,
            ball_velocity_y: ball.velocity_y as f32,
            paddle_x: paddle.x as f32,
            paddle_y: paddle.y as f32,
            block_hit,
        });
    }

    let file = File::create("/Users/luisa/vara-arkanoid/visualization/game_steps.json").expect("Unable to create file");
    serde_json::to_writer(file, &game_steps).expect("Unable to write data to file");

}
#[derive(Serialize)]
struct GameStep {
    ball_x: f32,
    ball_y: f32,
    ball_velocity_x: f32,
    ball_velocity_y: f32,
    paddle_x: f32,
    paddle_y: f32,
    block_hit: Option<(f32, f32)>,
}


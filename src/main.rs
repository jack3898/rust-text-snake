mod canvas;
mod config;
mod coordinates;
mod game;
mod renderer;
mod tasks;

use std::sync::{Arc, Mutex};

use config::PLAYFIELD_SIZE;
use game::Game;
use tasks::{game_loop_task, input_handler_task, render_task};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let game = Arc::new(Mutex::new(Game::new(PLAYFIELD_SIZE, PLAYFIELD_SIZE)));

    let (canvas_sender, canvas_receiver) = mpsc::channel(1);
    let sender_instance = mpsc::Sender::clone(&canvas_sender);

    let _ = tokio::join!(
        render_task(canvas_receiver),
        game_loop_task(Arc::clone(&game), sender_instance),
        input_handler_task(Arc::clone(&game))
    );
}

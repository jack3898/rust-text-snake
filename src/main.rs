mod canvas;
mod characters;
mod game;
mod renderer;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use canvas::Canvas;
use characters::Characters;
use crossterm::event::{read, Event, KeyCode};
use game::{Direction, Game};
use renderer::Renderer;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (canvas_sender, mut canvas_receiver) = mpsc::channel(1);
    let game = Arc::new(Mutex::new(Game::new()));

    let render_task = tokio::spawn(async move {
        let renderer = Renderer::new();

        loop {
            let canvas: Canvas = canvas_receiver.recv().await.unwrap();

            renderer.clear();
            renderer.render(&canvas);
        }
    });

    let sender_instance = mpsc::Sender::clone(&canvas_sender);
    let game_loop_game = Arc::clone(&game);

    let game_loop = tokio::spawn(async move {
        loop {
            let mut canvas = Canvas::new(15, 15, Characters::Grass);

            {
                let mut game = game_loop_game.lock().unwrap();

                game.next();

                let snake = game.get_snake();

                canvas.reset();

                for (snake_x, snake_y) in snake {
                    canvas.set_coord(*snake_x, *snake_y, Characters::SnakeBody.value());
                }

                canvas.set_coord(
                    snake.last().unwrap().0,
                    snake.last().unwrap().1,
                    Characters::SnakeHead.value(),
                );
            }

            sender_instance.send(canvas).await.unwrap();

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    let input_handler_game = Arc::clone(&game);

    let input_handler = tokio::spawn(async move {
        loop {
            if let Ok(Event::Key(key_event)) = read() {
                let mut game = input_handler_game.lock().unwrap();

                match key_event.code {
                    KeyCode::Up => game.set_snake_direction(Direction::Up),
                    KeyCode::Left => game.set_snake_direction(Direction::Left),
                    KeyCode::Down => game.set_snake_direction(Direction::Down),
                    KeyCode::Right => game.set_snake_direction(Direction::Right),
                    KeyCode::Char('r') => game.eat_apple(),
                    _ => (),
                }
            }
        }
    });

    let _ = tokio::join!(render_task, game_loop, input_handler);
}

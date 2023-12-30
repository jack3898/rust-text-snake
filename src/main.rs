mod canvas;
mod characters;
mod coordinate;
mod game;
mod renderer;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use canvas::Canvas;
use characters::Characters;
use crossterm::event::{read, Event, KeyCode};
use game::{
    apple::Apple,
    game::Game,
    game_state::GameState,
    powerup::PowerupType,
    snake::{Snake, SnakeDirection},
    supersnake::Supersnake,
};
use renderer::Renderer;
use tokio::sync::mpsc;

const SIZE: usize = 15;
const FRAME_TIME_MILLI: u64 = 150;

#[tokio::main]
async fn main() {
    let (canvas_sender, mut canvas_receiver) = mpsc::channel(1);
    let game = Arc::new(Mutex::new(Game::new(SIZE, SIZE)));

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
        let mut speed = FRAME_TIME_MILLI;

        loop {
            let canvas = {
                let mut game = game_loop_game.lock().unwrap();
                let game_state = game.next();

                match game_state {
                    GameState::Playing => {
                        let mut canvas = Canvas::new();
                        let score = game.get_score();
                        let apple = game.get_apple();
                        let supersnake = game.get_supersnake();
                        let snake_body = game.snake_get_body();
                        let snake_head = game.snake_get_head().unwrap();

                        canvas.fill(Characters::Grass.value(), SIZE, SIZE);

                        let score_render: Vec<char> = format!("Score: {}", score).chars().collect();
                        canvas.add_row(score_render);

                        let speed_display: Vec<char> =
                            format!("Speed: {}", (10000 / speed) - 66).chars().collect();
                        canvas.add_row(speed_display);

                        let powerup_display: Vec<char> = format!(
                            "Powerup ticks: {}",
                            match game.get_powerup() {
                                PowerupType::Supersnake { duration } => duration.to_string(),
                                PowerupType::None => "No powerup active".to_string(),
                            }
                        )
                        .chars()
                        .collect();

                        canvas.add_row(powerup_display);

                        for coordinate in snake_body {
                            match game.get_powerup() {
                                PowerupType::Supersnake { .. } => canvas
                                    .set_coord(&coordinate, Characters::SnakeBodySuper.value()),
                                PowerupType::None => {
                                    canvas.set_coord(&coordinate, Characters::SnakeBody.value())
                                }
                            };
                        }

                        canvas.set_coord(&snake_head, Characters::SnakeHead.value());

                        if let Some(apple) = apple {
                            canvas.set_coord(&apple, Characters::Apple.value());
                        };

                        if let Some(supersnake) = supersnake {
                            canvas.set_coord(&supersnake, Characters::Supersnake.value());
                        };

                        speed = if speed > 30 {
                            FRAME_TIME_MILLI - score as u64
                        } else {
                            speed
                        };

                        canvas
                    }
                    GameState::GameOver { score, message } => {
                        let mut canvas = Canvas::new();

                        let message: Vec<char> = message.chars().collect();
                        canvas.add_row(message);

                        let score: Vec<char> = format!("Final score: {}", score).chars().collect();
                        canvas.add_row(score);

                        canvas
                    }
                    GameState::Paused => {
                        let mut canvas = Canvas::new();

                        let message: Vec<char> = "Paused. Press 'r' to resume.".chars().collect();
                        canvas.add_row(message);

                        canvas
                    }
                }
            };

            sender_instance.send(canvas).await.unwrap();

            tokio::time::sleep(Duration::from_millis(speed)).await;
        }
    });

    let input_handler_game = Arc::clone(&game);

    let input_handler = tokio::spawn(async move {
        loop {
            if let Ok(Event::Key(key_event)) = read() {
                let mut game = input_handler_game.lock().unwrap();

                match game.get_state() {
                    GameState::Playing => {
                        // The matches! statements are used to stop people from accidentally eating the snake
                        match key_event.code {
                            KeyCode::Up => {
                                if !matches!(game.snake_get_direction(), SnakeDirection::Down) {
                                    game.snake_set_direction(SnakeDirection::Up)
                                }
                            }
                            KeyCode::Left => {
                                if !matches!(game.snake_get_direction(), SnakeDirection::Right) {
                                    game.snake_set_direction(SnakeDirection::Left)
                                }
                            }
                            KeyCode::Down => {
                                if !matches!(game.snake_get_direction(), SnakeDirection::Up) {
                                    game.snake_set_direction(SnakeDirection::Down)
                                }
                            }
                            KeyCode::Right => {
                                if !matches!(game.snake_get_direction(), SnakeDirection::Left) {
                                    game.snake_set_direction(SnakeDirection::Right)
                                }
                            }
                            KeyCode::Esc => {
                                game.pause();
                            }
                            _ => (),
                        }
                    }
                    GameState::GameOver { .. } => match key_event.code {
                        KeyCode::Char('r') => {
                            game.start_over();
                        }
                        _ => (),
                    },
                    GameState::Paused => match key_event.code {
                        KeyCode::Char('r') => {
                            game.resume();
                        }
                        _ => (),
                    },
                }
            }
        }
    });

    let _ = tokio::join!(render_task, game_loop, input_handler);
}

mod canvas;
mod characters;
mod coordinates;
mod game;
mod renderer;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use canvas::Canvas;
use characters::Characters;
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use game::{
    entity_type::EntityType,
    game::Game,
    game_state::GameState,
    powerup::PowerupType,
    traits::{
        entity::Entity,
        snake::{Snake, SnakeDirection},
    },
};
use renderer::Renderer;
use tokio::sync::mpsc;

const SIZE: usize = 21;
const FRAME_TIME_MILLI: u64 = 200;

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
                        let snake_body = game.snake_get_body();
                        let snake_head = game.snake_get_head().unwrap();

                        canvas.fill(Characters::Grass.value(), SIZE, SIZE);

                        let score_render: Vec<char> = format!("Score: {}", score).chars().collect();
                        canvas.add_row(score_render);

                        let speed_display: Vec<char> =
                            format!("Tick speed (ms): {}", speed).chars().collect();
                        canvas.add_row(speed_display);

                        let powerup_display: Vec<char> = format!(
                            "Powerup ticks: {}",
                            match game.get_current_powerup() {
                                PowerupType::Supersnake { duration } => duration.to_string(),
                                PowerupType::Slowdown { duration } => duration.to_string(),
                                PowerupType::None => "No powerup active".to_string(),
                            }
                        )
                        .chars()
                        .collect();

                        canvas.add_row(powerup_display);

                        for entity in game.get_all_entities() {
                            match entity {
                                EntityType::SupersnakePwrup { .. } => {
                                    canvas.set_coord(
                                        entity.get_coordinates().unwrap(),
                                        Characters::SupersnakePwrup.value(),
                                    );
                                }
                                EntityType::SlowdownPwrup { .. } => {
                                    canvas.set_coord(
                                        entity.get_coordinates().unwrap(),
                                        Characters::SlowdownPwrup.value(),
                                    );
                                }
                                EntityType::Apple { .. } => {
                                    canvas.set_coord(
                                        entity.get_coordinates().unwrap(),
                                        Characters::Apple.value(),
                                    );
                                }
                                EntityType::Obstacle { .. } => {
                                    canvas.set_coord(
                                        entity.get_coordinates().unwrap(),
                                        Characters::Obstacle.value(),
                                    );
                                }
                            };
                        }

                        for coordinate in snake_body {
                            match game.get_current_powerup() {
                                PowerupType::Supersnake { .. } => canvas
                                    .set_coord(&coordinate, Characters::SnakeBodySuper.value()),
                                PowerupType::Slowdown { .. } => {
                                    canvas.set_coord(&coordinate, Characters::SnakeBodySlow.value())
                                }
                                PowerupType::None => {
                                    canvas.set_coord(&coordinate, Characters::SnakeBody.value())
                                }
                            };
                        }

                        canvas.set_coord(&snake_head, Characters::SnakeHead.value());

                        if matches!(game.get_current_powerup(), PowerupType::Slowdown { .. }) {
                            speed = FRAME_TIME_MILLI + 50;
                        } else if speed > 60 {
                            speed = FRAME_TIME_MILLI - (score * 2) as u64
                        };

                        canvas
                    }
                    GameState::Intro => {
                        let mut canvas = Canvas::new();
                        let snake_display = format!(
                            "{}{}{}",
                            Characters::SnakeBody.value(),
                            Characters::SnakeBody.value(),
                            Characters::SnakeHead.value()
                        );
                        let apple_guide = format!("{} - Eat to grow.", Characters::Apple.value());

                        let supersnake_guide = format!(
                            "{} - Eat to become invincible to obstacles.",
                            Characters::SupersnakePwrup.value()
                        );
                        let slowdown_guide = format!("{} - Eat to slow down time, but if you spam-press the arrow key they corresponds to your current direction time speeds up!", Characters::SlowdownPwrup.value());
                        let obstacle_guide =
                            format!("{} - Avoid or game over!", Characters::Obstacle.value());

                        let messages = vec![
                            "Welcome to Snake!",
                            snake_display.as_str(),
                            "Use the arrow keys to move.",
                            "",
                            apple_guide.as_str(),
                            supersnake_guide.as_str(),
                            slowdown_guide.as_str(),
                            obstacle_guide.as_str(),
                            "",
                            "Press [SPACE] to start.",
                            "You can quit at any time by pressing [ESC] in this screen.",
                        ];

                        for message in messages {
                            let message: Vec<char> = message.chars().collect();
                            canvas.add_row(message);
                        }

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
                }
            };

            sender_instance.send(canvas).await.unwrap();

            tokio::time::sleep(Duration::from_millis(speed)).await;
        }
    });

    let input_handler_game = Arc::clone(&game);

    let input_handler = tokio::spawn(async move {
        let mut last_key = KeyCode::Null;

        loop {
            if let Ok(Event::Key(key_event)) = read() {
                let mut game = input_handler_game.lock().unwrap();

                // This thread polls very quickly, so we need to make sure we don't poll the same key twice
                match key_event.kind {
                    KeyEventKind::Press if (key_event.code == last_key) => continue,
                    KeyEventKind::Press => last_key = key_event.code,
                    KeyEventKind::Repeat => continue,
                    KeyEventKind::Release => {
                        last_key = KeyCode::Null;

                        continue;
                    }
                }

                match game.get_state() {
                    GameState::Playing => {
                        // The matches! statements are used to stop people from accidentally eating the snake
                        match key_event.code {
                            KeyCode::Up => game.snake_set_direction(SnakeDirection::Up),
                            KeyCode::Left => game.snake_set_direction(SnakeDirection::Left),
                            KeyCode::Down => game.snake_set_direction(SnakeDirection::Down),
                            KeyCode::Right => game.snake_set_direction(SnakeDirection::Right),
                            _ => (),
                        }
                    }
                    GameState::Intro => match key_event.code {
                        KeyCode::Char(' ') => {
                            game.play();
                        }
                        KeyCode::Esc => {
                            println!("Thanks for playing!");
                            std::process::exit(0);
                        }
                        _ => (),
                    },
                    GameState::GameOver { .. } => match key_event.code {
                        KeyCode::Char('r') => {
                            game.start_over();
                        }
                        _ => (),
                    },
                }
            }
        }
    });

    let _ = tokio::join!(render_task, game_loop, input_handler);
}

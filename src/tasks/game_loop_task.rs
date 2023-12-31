use std::{sync::Arc, time::Duration};

use tokio::{sync::mpsc, task::JoinHandle};

use crate::{
    canvas::Canvas,
    config::PLAYFIELD_SIZE,
    game::{Characters, Entity, EntityType, Game, GameState, PowerupType, Snake},
};

pub fn game_loop_task(
    game: Arc<std::sync::Mutex<Game>>,
    sender_instance: mpsc::Sender<Canvas>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let (canvas, tick_speed) = {
                let mut game = game.lock().unwrap();
                let game_state = game.next();

                let final_canvas = match game_state {
                    GameState::Playing => {
                        let mut canvas = Canvas::new();

                        let score = game.get_score();
                        let snake_body = game.snake_get_body();
                        let snake_head = game.snake_get_head().unwrap();

                        let score_render = format!("Score: {}", score);
                        let speed_display = format!("Tick speed (ms): {}", game.get_tick_speed());

                        let powerup_display = format!(
                            "Powerup ticks: {}",
                            match game.get_current_powerup() {
                                PowerupType::Supersnake { tick_duration } =>
                                    tick_duration.to_string(),
                                PowerupType::Slowdown { tick_duration } =>
                                    tick_duration.to_string(),
                                PowerupType::None => "No powerup active".to_string(),
                            }
                        );

                        canvas.fill(Characters::Grass.value(), PLAYFIELD_SIZE, PLAYFIELD_SIZE);
                        canvas.add_row_string(score_render);
                        canvas.add_row_string(speed_display);
                        canvas.add_row_string(powerup_display);

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

                        canvas
                    }
                    GameState::Intro => {
                        let mut canvas = Canvas::new();

                        canvas.add_row_string("Welcome to Snake!");
                        canvas.add_row_string(format!(
                            "{}{}{}",
                            Characters::SnakeBody.value(),
                            Characters::SnakeBody.value(),
                            Characters::SnakeHead.value()
                        ));
                        canvas.add_row_string("Use the arrow keys to move.");
                        canvas.add_row_string("");
                        canvas.add_row_string(format!(
                            "{} - Eat to grow.",
                            Characters::Apple.value()
                        ));
                        canvas.add_row_string(format!(
                            "{} - Eat to become invincible to obstacles.",
                            Characters::SupersnakePwrup.value()
                        ));
                        canvas.add_row_string(format!("{} - Eat to slow down time, but if you spam-press the arrow key they corresponds to your current direction time speeds up!", Characters::SlowdownPwrup.value()));
                        canvas.add_row_string(format!(
                            "{} - Avoid or game over!",
                            Characters::Obstacle.value()
                        ));
                        canvas.add_row_string("");
                        canvas.add_row_string("Press [SPACE] to start.");
                        canvas.add_row_string(
                            "You can quit at any time by pressing [ESC] in this screen.",
                        );

                        canvas
                    }
                    GameState::GameOver { score, message } => {
                        let mut canvas = Canvas::new();

                        canvas.add_row_string(message);
                        canvas.add_row_string(format!("Final score: {}", score));

                        canvas
                    }
                };

                (final_canvas, game.get_tick_speed())
            };

            sender_instance.send(canvas).await.unwrap();

            tokio::time::sleep(Duration::from_millis(tick_speed)).await;
        }
    })
}

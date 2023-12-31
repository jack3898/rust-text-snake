use std::sync::Arc;

use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use tokio::task::JoinHandle;

use crate::game::{Game, GameState, Snake, SnakeDirection};

pub fn input_handler_task(game: Arc<std::sync::Mutex<Game>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut last_key = KeyCode::Null;

        loop {
            if let Ok(Event::Key(key_event)) = read() {
                let mut game = game.lock().unwrap();

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
    })
}

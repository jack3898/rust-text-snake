mod canvas;
mod characters;
mod game;
mod renderer;

use std::time::Duration;

use canvas::Canvas;
use characters::Characters;
use game::Game;
use renderer::Renderer;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(1);

    let render_task = tokio::spawn(async move {
        let renderer = Renderer::new();

        loop {
            let canvas: Canvas = receiver.recv().await.unwrap();

            renderer.clear();
            renderer.render(&canvas);
        }
    });

    let sender_instance = mpsc::Sender::clone(&sender);

    let game_loop = tokio::spawn(async move {
        let mut game = Game::new();

        loop {
            game.next();

            let mut canvas = Canvas::new(15, 15, Characters::Grass);
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

            sender_instance.send(canvas).await.unwrap();

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    let _ = tokio::join!(render_task, game_loop);
}

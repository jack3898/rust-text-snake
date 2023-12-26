mod canvas;
mod renderer;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use canvas::Canvas;
use renderer::Renderer;

#[tokio::main]
async fn main() {
    let canvas = Arc::new(Mutex::new(Canvas::new(15, 15, '🟩')));
    let renderer = Renderer::new(Arc::clone(&canvas));

    let render_task = tokio::spawn(async move {
        loop {
            renderer.clear();
            renderer.render();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 0..10 {
        for j in 0..10 {
            canvas.lock().unwrap().update_coord(i, j, '🟨');

            thread::sleep(Duration::from_millis(300));
        }
    }

    let _ = render_task.await;
}

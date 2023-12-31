use tokio::{sync::mpsc::Receiver, task::JoinHandle};

use crate::{canvas::Canvas, renderer::Renderer};

pub fn render_task(mut canvas_receiver: Receiver<Canvas>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let renderer = Renderer::new();

        loop {
            let canvas: Canvas = canvas_receiver.recv().await.unwrap();

            renderer.clear();
            renderer.render(&canvas);
        }
    })
}

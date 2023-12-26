use std::{process::Command, sync::Arc};

use crate::canvas::Canvas;

pub struct Renderer {
    canvas: Arc<std::sync::Mutex<Canvas>>,
}

impl Renderer {
    pub fn new(canvas: Arc<std::sync::Mutex<Canvas>>) -> Self {
        Self { canvas }
    }

    pub fn render(&self) {
        let buf = self.canvas.lock().unwrap().to_buffer();

        println!("{buf}");
    }

    pub fn clear(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
    }
}

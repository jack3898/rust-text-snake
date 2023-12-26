use std::process::Command;

use crate::canvas::Canvas;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, canvas: &Canvas) {
        let buf = canvas.to_buffer();

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

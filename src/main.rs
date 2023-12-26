mod renderer;

use std::{thread, time::Duration};

use renderer::Renderer;

fn main() {
    let renderer = Renderer::new(20, 20);

    loop {
        renderer.clear();
        renderer.render();
        thread::sleep(Duration::from_millis(500));
    }
}

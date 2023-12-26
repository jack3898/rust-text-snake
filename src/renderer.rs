use std::process::Command;

pub struct Renderer {
    matrix: Vec<Vec<char>>,
}

impl Renderer {
    pub fn new(x_res: usize, y_res: usize) -> Self {
        Self {
            matrix: vec![vec!['0'; y_res]; x_res],
        }
    }

    pub fn buffer(&self) -> String {
        let mut buf = String::new();

        for row in &self.matrix {
            buf.push('|');

            for val in row {
                buf.push(*val);
            }

            buf.push('|');
            buf.push('\n');
        }

        buf
    }

    pub fn render(&self) {
        let buf = self.buffer();

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

#[cfg(test)]
mod tests {
    use super::Renderer;

    #[test]
    fn should_create_correct_buffer() {
        let renderer = Renderer::new(3, 3);
        let buf = renderer.buffer();

        assert_eq!(buf, "\0\0\0\n\0\0\0\n\0\0\0\n")
    }
}

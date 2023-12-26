pub struct Canvas {
    matrix: Vec<Vec<char>>,
}

impl Canvas {
    pub fn new(x_res: usize, y_res: usize, fill_with: char) -> Self {
        Self {
            matrix: vec![vec![fill_with; y_res]; x_res],
        }
    }

    pub fn to_buffer(&self) -> String {
        let mut buf = String::new();

        for row in &self.matrix {
            for val in row {
                buf.push(*val);
            }

            buf.push('\n');
        }

        buf
    }

    pub fn update_coord(&mut self, x: usize, y: usize, new_char: char) {
        self.matrix[x][y] = new_char;
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;

    #[test]
    fn should_create_correct_buffer() {
        let canvas = Canvas::new(3, 3, '🟩');
        let buf = canvas.to_buffer();

        assert_eq!(buf, "🟩🟩🟩\n🟩🟩🟩\n🟩🟩🟩\n")
    }
}

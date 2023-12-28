#[derive(Clone)]
pub struct Canvas {
    matrix: Vec<Vec<char>>,
    x_res: usize,
    y_res: usize,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            matrix: vec![],
            x_res: 0,
            y_res: 0,
        }
    }

    pub fn fill(&mut self, char: char, x_res: usize, y_res: usize) {
        self.matrix = vec![vec![char; y_res]; x_res];
        self.x_res = x_res;
        self.y_res = y_res;
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

    pub fn set_coord(&mut self, x: usize, y: usize, new_char: char) -> bool {
        self.matrix.get_mut(y).is_some_and(|row| {
            row.get_mut(x).is_some_and(|value| {
                *value = new_char;

                true
            })
        })
    }

    pub fn add_row(&mut self, row: Vec<char>) {
        self.matrix.push(row);
        self.y_res += 1;
    }
}

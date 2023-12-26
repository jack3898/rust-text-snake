use crate::characters::Characters;

#[derive(Clone)]
pub struct Canvas {
    matrix: Vec<Vec<char>>,
    backdrop_char: Characters,
    x_res: usize,
    y_res: usize,
}

impl Canvas {
    pub fn new(x_res: usize, y_res: usize, backdrop_char: Characters) -> Self {
        Self {
            matrix: vec![vec![backdrop_char.value(); y_res]; x_res],
            backdrop_char,
            x_res,
            y_res,
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

    pub fn set_coord(&mut self, x: usize, y: usize, new_char: char) -> bool {
        self.matrix.get_mut(y).is_some_and(|row| {
            row.get_mut(x).is_some_and(|value| {
                *value = new_char;

                true
            })
        })
    }

    pub fn reset(&mut self) {
        self.matrix = vec![vec![self.backdrop_char.value(); self.y_res]; self.x_res];
    }
}

#[cfg(test)]
mod tests {
    use crate::characters::Characters;

    use super::Canvas;

    #[test]
    fn should_create_correct_buffer() {
        let canvas = Canvas::new(3, 3, Characters::Grass);
        let buf = canvas.to_buffer();

        let grs = Characters::Grass.value();

        assert_eq!(
            buf,
            format!("{grs}{grs}{grs}\n{grs}{grs}{grs}\n{grs}{grs}{grs}\n")
        )
    }
}

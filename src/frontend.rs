use game::{MineSweeper, Position};
use piston_window::*;

pub struct Gui {
    game: MineSweeper,
    square_size: u32,
    selected_position: Option<Position>,
}

impl Gui {
    pub fn new() -> Gui {
        Gui {
            game: MineSweeper::new(9, 9, 10),
            square_size: 30,
            selected_position: None,
        }
    }

    // convention [width, height]
    pub fn get_window_size(&self) -> [u32; 2] {
        [
            self.game.cols * self.square_size,
            self.game.rows * self.square_size,
        ]
    }

    pub fn handle_mouse_position(&mut self, position: [f64; 2]) {
        if position[0] >= 0.0 && position[1] >= 0.0
            && position[0] < ((self.game.cols * self.square_size) as f64)
            && position[1] < ((self.game.rows * self.square_size) as f64)
        {
            self.selected_position = Some(Position(
                position[1] as u32 / self.square_size,
                position[0] as u32 / self.square_size,
            ));
        } else {
            self.selected_position = None;
        }
    }

    pub fn draw(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            // hard coded 2 pixel offset
            let curr_trans = c.transform.trans(2.0, 2.0);

            for i in 0..self.game.rows {
                for j in 0..self.game.cols {
                    let curr_x = j * self.square_size;
                    let curr_y = i * self.square_size;
                    rectangle::Rectangle::new_border([0.8, 0.8, 0.8, 1.0], 1.0).draw(
                        [
                            curr_x as f64,
                            curr_y as f64,
                            (self.square_size as f64) - 4.0,
                            (self.square_size as f64) - 4.0,
                        ],
                        &Default::default(),
                        curr_trans,
                        g,
                    );
                }
            }
        });
    }
}

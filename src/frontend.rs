use game::{GameState, MineSweeper, Position, SquareState};
use piston_window::rectangle::Border;
use piston_window::*;

pub struct Gui {
    game: MineSweeper,
    square_size: u32,
    top_bar_height: u32,
    selected_position: Option<Position>,
}

impl Gui {
    pub fn new() -> Gui {
        Gui {
            game: MineSweeper::new(8, 10, 10),
            square_size: 30,
            top_bar_height: 40,
            selected_position: None,
        }
    }

    // convention [width, height]
    pub fn get_window_size(&self) -> [u32; 2] {
        [
            self.game.cols * self.square_size,
            self.game.rows * self.square_size + self.top_bar_height,
        ]
    }

    pub fn handle_mouse_position(&mut self, x: f64, mut y: f64) {
        y -= self.top_bar_height as f64;

        if x >= 0.0 && y >= 0.0 && x < ((self.game.cols * self.square_size) as f64)
            && y < ((self.game.rows * self.square_size) as f64)
        {
            self.selected_position = Some(Position(
                y as u32 / self.square_size,
                x as u32 / self.square_size,
            ));
        } else {
            self.selected_position = None;
        }
    }

    pub fn handle_mouse_click(&mut self, button: MouseButton) {
        if self.game.state == GameState::Ongoing && self.selected_position.is_some() {
            match button {
                MouseButton::Left => {
                    self.game.reveal_square(&self.selected_position.unwrap());
                }
                MouseButton::Right => {
                    self.game
                        .toggle_flag_square(&self.selected_position.unwrap());
                }
                _ => (),
            }
        }

        self.game.update_game_state();
    }

    pub fn draw(&self, window: &mut PistonWindow, event: &Event, glyphs: &mut Glyphs) {
        window.draw_2d(event, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            // hard coded 2 pixel offset
            let board_transform = c.transform.trans(2.0, 2.0 + self.top_bar_height as f64);

            for i in 0..self.game.rows {
                for j in 0..self.game.cols {
                    let curr_x = j * self.square_size;
                    let curr_y = i * self.square_size;

                    let curr_square = self.game.get_square(i, j);

                    match curr_square.state {
                        SquareState::Covered => {
                            rectangle::Rectangle::new_border([0.8, 0.8, 0.8, 1.0], 1.0)
                                .color([0.7, 0.7, 0.7, 1.0])
                                .draw(
                                    [
                                        curr_x as f64,
                                        curr_y as f64,
                                        (self.square_size as f64) - 4.0,
                                        (self.square_size as f64) - 4.0,
                                    ],
                                    &Default::default(),
                                    board_transform,
                                    g,
                                );
                        }
                        SquareState::Revealed => {
                            let mut rect = rectangle::Rectangle::new([0.0, 0.0, 0.0, 0.0]);

                            if curr_square.is_mine {
                                rect = rect.color([0.9, 0.0, 0.0, 1.0]).border(Border {
                                    color: [0.8, 0.0, 0.0, 1.0],
                                    radius: 1.0,
                                });
                            } else {
                                rect = rect.color([0.9, 0.9, 0.9, 1.0]).border(Border {
                                    color: [0.8, 0.8, 0.8, 1.0],
                                    radius: 1.0,
                                });
                            }
                            rect.draw(
                                [
                                    curr_x as f64,
                                    curr_y as f64,
                                    (self.square_size as f64) - 4.0,
                                    (self.square_size as f64) - 4.0,
                                ],
                                &Default::default(),
                                board_transform,
                                g,
                            );
                        }
                        SquareState::Flagged => (),
                    }

                    let text_transform = board_transform.trans(
                        curr_x as f64 + self.square_size as f64 / 3.5,
                        curr_y as f64 + self.square_size as f64 / 1.5,
                    );

                    if curr_square.is_mine {
                        text([0.0, 0.0, 0.0, 1.0], 20, "*", glyphs, text_transform, g).unwrap();
                    }

                    if !curr_square.is_mine && curr_square.adjacent_mines > 0 {
                        text(
                            [0.0, 0.0, 0.0, 1.0],
                            20,
                            &curr_square.adjacent_mines.to_string(),
                            glyphs,
                            text_transform,
                            g,
                        ).unwrap();
                    }
                }
            }
        });
    }
}

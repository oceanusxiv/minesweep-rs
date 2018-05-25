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
            game: MineSweeper::new(9, 9, 10),
            square_size: 30,
            top_bar_height: 35,
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
        y -= f64::from(self.top_bar_height);

        if x >= 0.0 && y >= 0.0 && x < f64::from(self.game.cols * self.square_size)
            && y < f64::from(self.game.rows * self.square_size)
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

    pub fn handle_key_press(&mut self, key: Key) {
        match key {
            Key::R => self.game.reset(),
            _ => (),
        }
    }

    fn get_text_color(num: u32) -> [f32; 4] {
        let color;
        match num {
            1 => color = [0.0, 0.0, 1.0, 1.0],
            2 => color = [0.0, 1.0, 0.0, 1.0],
            3 => color = [1.0, 0.0, 0.0, 1.0],
            4 => color = [0.5, 0.0, 0.5, 1.0],
            5 => color = [0.5, 0.0, 0.0, 1.0],
            6 => color = [0.0, 1.0, 1.0, 1.0],
            7 => color = [0.0, 0.0, 0.0, 1.0],
            8 => color = [0.5, 0.5, 0.5, 1.0],
            _ => color = [0.0, 0.0, 0.0, 1.0],
        }

        color
    }

    pub fn draw(
        &self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        mine: &G2dTexture,
        flag: &G2dTexture,
        win_face: &G2dTexture,
        ongoing_face: &G2dTexture,
        lost_face: &G2dTexture,
    ) {
        window.draw_2d(event, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            rectangle::Rectangle::new([0.3, 0.3, 0.3, 1.0]).draw(
                [3.0, 3.0, 50.0, 30.0],
                &Default::default(),
                c.transform,
                g,
            );

            let flag_num_transform = c.transform.trans(4.8, 28.0).zoom(0.5);

            text(
                [1.0, 0.46, 0.35, 1.0],
                60,
                &format!("{:03}", self.game.get_flags_left()),
                glyphs,
                flag_num_transform,
                g,
            ).unwrap();

            let face_transform = c.transform
                .trans(
                    f64::from(self.game.cols * self.square_size) * 0.5 - 12.8,
                    5.0,
                )
                .zoom(0.2);

            match self.game.state {
                GameState::Ongoing => image(ongoing_face, face_transform, g),
                GameState::Won => image(win_face, face_transform, g),
                GameState::Lost => image(lost_face, face_transform, g),
            }

            // hard coded 2 pixel offset
            let board_transform = c.transform.trans(2.0, 2.0 + f64::from(self.top_bar_height));

            for i in 0..self.game.rows {
                for j in 0..self.game.cols {
                    let curr_x = j * self.square_size;
                    let curr_y = i * self.square_size;

                    let curr_square = self.game.get_square(i, j);

                    let text_transform = board_transform
                        .trans(
                            f64::from(curr_x) + f64::from(self.square_size) * 0.28,
                            f64::from(curr_y) + f64::from(self.square_size) * 0.66,
                        )
                        .zoom(0.5);

                    let mine_transform = board_transform
                        .trans(
                            f64::from(curr_x) + f64::from(self.square_size) * 0.1,
                            f64::from(curr_y) + f64::from(self.square_size) * 0.1,
                        )
                        .zoom(0.1);

                    match curr_square.state {
                        SquareState::Covered => {
                            rectangle::Rectangle::new_border([0.8, 0.8, 0.8, 1.0], 1.0)
                                .color([0.9, 0.9, 0.9, 1.0])
                                .draw(
                                    [
                                        f64::from(curr_x),
                                        f64::from(curr_y),
                                        f64::from(self.square_size) - 4.0,
                                        f64::from(self.square_size) - 4.0,
                                    ],
                                    &Default::default(),
                                    board_transform,
                                    g,
                                );
                        }
                        SquareState::Revealed => {
                            let mut rect = rectangle::Rectangle::new([0.0, 0.0, 0.0, 0.0]);

                            if curr_square.is_mine {
                                rect = rect.color([0.7, 0.0, 0.0, 1.0]).border(Border {
                                    color: [0.8, 0.0, 0.0, 1.0],
                                    radius: 1.0,
                                });
                            } else {
                                rect = rect.color([0.7, 0.7, 0.7, 1.0]).border(Border {
                                    color: [0.8, 0.8, 0.8, 1.0],
                                    radius: 1.0,
                                });
                            }

                            rect.draw(
                                [
                                    f64::from(curr_x),
                                    f64::from(curr_y),
                                    f64::from(self.square_size) - 4.0,
                                    f64::from(self.square_size) - 4.0,
                                ],
                                &Default::default(),
                                board_transform,
                                g,
                            );

                            if curr_square.is_mine {
                                image(mine, mine_transform, g);
                            }

                            if !curr_square.is_mine && curr_square.adjacent_mines > 0 {
                                text(
                                    Gui::get_text_color(curr_square.adjacent_mines),
                                    40,
                                    &curr_square.adjacent_mines.to_string(),
                                    glyphs,
                                    text_transform,
                                    g,
                                ).unwrap();
                            }
                        }
                        SquareState::Flagged => {
                            rectangle::Rectangle::new_border([0.8, 0.8, 0.8, 1.0], 1.0)
                                .color([0.9, 0.9, 0.9, 1.0])
                                .draw(
                                    [
                                        f64::from(curr_x),
                                        f64::from(curr_y),
                                        f64::from(self.square_size) - 4.0,
                                        f64::from(self.square_size) - 4.0,
                                    ],
                                    &Default::default(),
                                    board_transform,
                                    g,
                                );

                            let flag_transform = board_transform
                                .trans(
                                    f64::from(curr_x) + f64::from(self.square_size) * 0.1,
                                    f64::from(curr_y) + f64::from(self.square_size) * 0.1,
                                )
                                .zoom(0.15);

                            image(flag, flag_transform, g);
                        }
                    }
                }
            }
        });
    }
}

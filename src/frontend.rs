use game::{GameState, MineSweeper, Position, SquareState};
use piston_window::rectangle::Border;
use piston_window::*;

pub struct Icons {
    pub mine: G2dTexture,
    pub flag: G2dTexture,
    pub win_face: G2dTexture,
    pub ongoing_face: G2dTexture,
    pub lost_face: G2dTexture,
}

pub struct Gui {
    game: MineSweeper,
    square_size: u32,
    top_bar_height: u32,
    selected_position: Option<Position>,
}

impl Gui {
    pub fn new(cols: u32, rows: u32, num_mines: u32) -> Gui {
        Gui {
            game: MineSweeper::new(cols, rows, num_mines),
            square_size: 20,
            top_bar_height: 27,
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
                    self.game.first_moved();
                }
                MouseButton::Right => {
                    self.game
                        .toggle_flag_square(&self.selected_position.unwrap());
                    self.game.first_moved();
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
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        icons: &Icons,
    ) {
        let margin = 2.0;
        let ui_font_size: u32 = 40;
        let ui_font_vert = 22.0;
        let ui_rect_height = f64::from(self.top_bar_height) - 2.0*margin;
        window.draw_2d(event, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            rectangle::Rectangle::new([0.3, 0.3, 0.3, 1.0]).draw(
                [margin, margin, f64::from(ui_font_size)*1.15, ui_rect_height],
                &Default::default(),
                c.transform,
                g,
            );

            let flag_num_transform = c.transform.trans(3.5, ui_font_vert).zoom(0.5);

            text(
                [1.0, 0.46, 0.35, 1.0],
                ui_font_size,
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
                .zoom(0.14);

            match self.game.state {
                GameState::Ongoing => image(&icons.ongoing_face, face_transform, g),
                GameState::Won => image(&icons.win_face, face_transform, g),
                GameState::Lost => image(&icons.lost_face, face_transform, g),
            }

            let time_rect_width = f64::from(ui_font_size)*1.5;
            rectangle::Rectangle::new([0.3, 0.3, 0.3, 1.0]).draw(
                [
                    f64::from(self.game.cols * self.square_size) - time_rect_width - margin,
                    margin,
                    time_rect_width,
                    ui_rect_height,
                ],
                &Default::default(),
                c.transform,
                g,
            );

            let time_transform = c.transform
                .trans(f64::from(self.game.cols * self.square_size) - 60.0, ui_font_vert)
                .zoom(0.5);

            text(
                [1.0, 0.46, 0.35, 1.0],
                ui_font_size,
                &format!("{:04}", self.game.game_time()),
                glyphs,
                time_transform,
                g,
            ).unwrap();

            // hard coded 2 pixel offset
            let board_transform = c.transform.trans(2.0, 2.0 + f64::from(self.top_bar_height));

            for i in 0..self.game.rows {
                for j in 0..self.game.cols {
                    let curr_x = j * self.square_size;
                    let curr_y = i * self.square_size;

                    let curr_square = self.game.get_square(i, j);

                    let text_transform = board_transform
                        .trans(
                            f64::from(curr_x) + f64::from(self.square_size) * 0.19,
                            f64::from(curr_y) + f64::from(self.square_size) * 0.65,
                        )
                        .zoom(0.5);

                    let mine_transform = board_transform
                        .trans(
                            f64::from(curr_x) + f64::from(self.square_size) * 0.06,
                            f64::from(curr_y) + f64::from(self.square_size) * 0.06,
                        )
                        .zoom(0.07);

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
                                image(&icons.mine, mine_transform, g);
                            }

                            if !curr_square.is_mine && curr_square.adjacent_mines > 0 {
                                text(
                                    Gui::get_text_color(curr_square.adjacent_mines),
                                    23,
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
                                    f64::from(curr_x) + f64::from(self.square_size) * 0.085,
                                    f64::from(curr_y) + f64::from(self.square_size) * 0.085,
                                )
                                .zoom(0.10);

                            image(&icons.flag, flag_transform, g);
                        }
                    }
                }
            }
        });
    }
}

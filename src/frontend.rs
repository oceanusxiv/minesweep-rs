use game::{Difficulty, GameState, MineSweeper, Position, SquareState};
use piston_window::rectangle::Border;
use piston_window::*;

pub struct Icons {
    pub mine: G2dTexture,
    pub flag: G2dTexture,
    pub win_face: G2dTexture,
    pub ongoing_face: G2dTexture,
    pub lost_face: G2dTexture,
}

const TOP_BAR_HEIGHT: u32 = 27;
const SQUARE_SIZE: u32 = 20;
const MARGIN: f64 = 2.0;
const UI_FONT_SIZE: u32 = 40;
const UI_FONT_Y_OFFSET: f64 = 22.0;
const UI_RECT_HEIGHT: f64 = TOP_BAR_HEIGHT as f64 - 2.0 * MARGIN;
const BACKGROUND_COLOR: types::Color = [0.5, 0.5, 0.5, 1.0];
const CELL_BORDER_COLOR: types::Color = [0.8, 0.8, 0.8, 1.0];
const MINE_BORDER_COLOR: types::Color = [0.8, 0.0, 0.0, 1.0];
const CELL_COVERED_COLOR: types::Color = [0.9, 0.9, 0.9, 1.0];
const CELL_REVEALED_COLOR: types::Color = [0.7, 0.7, 0.7, 1.0];
const MINE_REVEALED_COLOR: types::Color = [0.7, 0.0, 0.0, 1.0];
const UI_RECT_COLOR: types::Color = [0.3, 0.3, 0.3, 1.0];
const UI_TEXT_COLOR: types::Color = [1.0, 0.46, 0.35, 1.0];
const FACE_ICON_SCALE: f64 = 0.14;

pub struct Gui {
    game: MineSweeper,
    selected_position: Option<Position>,
    face_selected: bool,
    left_mouse_pressed: bool,
    right_mouse_pressed: bool,
    custom_rows: u32,
    custom_cols: u32,
    custom_mines: u32,
    face_button_rect: [f64; 4],
    difficulty: Difficulty,
}

impl Gui {
    pub fn new(cols: u32, rows: u32, num_mines: u32, difficulty: Difficulty) -> Gui {
        Gui {
            game: MineSweeper::new(cols, rows, num_mines),
            selected_position: None,
            face_selected: false,
            left_mouse_pressed: false,
            right_mouse_pressed: false,
            custom_rows: rows,
            custom_cols: cols,
            custom_mines: num_mines,
            face_button_rect: [0.0, 0.0, 0.0, 0.0],
            difficulty,
        }
    }

    // convention [width, height]
    pub fn get_window_size(&self) -> [u32; 2] {
        [
            self.game.cols * SQUARE_SIZE,
            self.game.rows * SQUARE_SIZE + TOP_BAR_HEIGHT,
        ]
    }

    pub fn handle_mouse_position(&mut self, x: f64, y: f64) {
        // face button processing
        if x >= self.face_button_rect[0] && y >= self.face_button_rect[1]
            && (x <= self.face_button_rect[0] + self.face_button_rect[2])
            && (y <= self.face_button_rect[1] + self.face_button_rect[3])
        {
            self.face_selected = true;
        } else {
            self.face_selected = false;
        }

        let y_board = y - f64::from(TOP_BAR_HEIGHT);

        if x >= 0.0 && y_board >= 0.0 && x < f64::from(self.game.cols * SQUARE_SIZE)
            && y_board < f64::from(self.game.rows * SQUARE_SIZE)
        {
            self.selected_position = Some(Position(
                y_board as u32 / SQUARE_SIZE,
                x as u32 / SQUARE_SIZE,
            ));
        } else {
            self.selected_position = None;
        }
    }

    pub fn handle_mouse_click(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left_mouse_pressed = false,
            MouseButton::Right => self.right_mouse_pressed = false,
            _ => (),
        }
        if self.game.state == GameState::Ongoing && self.selected_position.is_some() {
            match button {
                MouseButton::Left => {
                    self.game.reveal_square(&self.selected_position.unwrap());
                    self.game.first_moved();
                }
                MouseButton::Right => {
                    self.game
                        .toggle_flag_square(&self.selected_position.unwrap());
                }
                _ => (),
            }
        }

        // face button processing
        if self.face_selected {
            self.game.reset();
        }

        self.game.update_game_state();
    }

    pub fn handle_mouse_press(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left_mouse_pressed = true,
            MouseButton::Right => self.right_mouse_pressed = true,
            _ => (),
        }
    }

    pub fn handle_key_press(&mut self, key: Key, window: &mut PistonWindow) {
        match key {
            Key::R => self.game.reset(),
            Key::D1 => {
                self.game = MineSweeper::new_from_preset(&Difficulty::Beginner);
                self.difficulty = Difficulty::Beginner;
                window.set_size(self.get_window_size());
            }
            Key::D2 => {
                self.game = MineSweeper::new_from_preset(&Difficulty::Intermediate);
                self.difficulty = Difficulty::Intermediate;
                window.set_size(self.get_window_size());
            }
            Key::D3 => {
                self.game = MineSweeper::new_from_preset(&Difficulty::Expert);
                self.difficulty = Difficulty::Expert;
                window.set_size(self.get_window_size());
            }
            Key::D4 => {
                self.game = MineSweeper::new(self.custom_cols, self.custom_rows, self.custom_mines);
                self.difficulty = Difficulty::Custom;
                window.set_size(self.get_window_size());
            }
            Key::Up => {
                match self.difficulty {
                    Difficulty::Beginner => {
                        self.game = MineSweeper::new_from_preset(&Difficulty::Intermediate);
                        self.difficulty = Difficulty::Intermediate;
                    }
                    Difficulty::Intermediate => {
                        self.game = MineSweeper::new_from_preset(&Difficulty::Expert);
                        self.difficulty = Difficulty::Expert;
                    }
                    Difficulty::Expert => {
                        self.game =
                            MineSweeper::new(self.custom_cols, self.custom_rows, self.custom_mines);
                        self.difficulty = Difficulty::Custom;
                    }
                    Difficulty::Custom => {
                        self.game = MineSweeper::new_from_preset(&Difficulty::Beginner);
                        self.difficulty = Difficulty::Beginner;
                    }
                }

                window.set_size(self.get_window_size());
            }
            Key::Down => {
                match self.difficulty {
                    Difficulty::Expert => {
                        self.game = MineSweeper::new_from_preset(&Difficulty::Intermediate);
                        self.difficulty = Difficulty::Intermediate;
                    }
                    Difficulty::Custom => {
                        self.game = MineSweeper::new_from_preset(&Difficulty::Expert);
                        self.difficulty = Difficulty::Expert;
                    }
                    Difficulty::Beginner => {
                        self.game =
                            MineSweeper::new(self.custom_cols, self.custom_rows, self.custom_mines);
                        self.difficulty = Difficulty::Custom;
                    }
                    Difficulty::Intermediate => {
                        self.game = MineSweeper::new_from_preset(&Difficulty::Beginner);
                        self.difficulty = Difficulty::Beginner;
                    }
                }

                window.set_size(self.get_window_size());
            }
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

    fn draw_face_button(
        &mut self,
        c: &Context,
        g: &mut G2d,
        x: f64,
        y: f64,
        icon_width: f64,
        icon_height: f64,
    ) {
        let color = if self.left_mouse_pressed && self.face_selected {
            CELL_REVEALED_COLOR
        } else {
            CELL_COVERED_COLOR
        };

        self.face_button_rect = [
            x - MARGIN,
            y - MARGIN,
            icon_width + MARGIN * 2.0,
            icon_height + MARGIN * 2.0,
        ];

        rectangle::Rectangle::new_border(CELL_BORDER_COLOR, 1.0)
            .color(color)
            .draw(self.face_button_rect, &Default::default(), c.transform, g);
    }

    pub fn draw(
        &mut self,
        window: &mut PistonWindow,
        event: &Event,
        glyphs: &mut Glyphs,
        icons: &Icons,
    ) {
        window.draw_2d(event, |c, g| {
            clear(BACKGROUND_COLOR, g);

            rectangle::Rectangle::new(UI_RECT_COLOR).draw(
                [
                    MARGIN,
                    MARGIN,
                    f64::from(UI_FONT_SIZE) * 1.15,
                    UI_RECT_HEIGHT,
                ],
                &Default::default(),
                c.transform,
                g,
            );

            let time_rect_width = f64::from(UI_FONT_SIZE) * 1.5;
            rectangle::Rectangle::new(UI_RECT_COLOR).draw(
                [
                    f64::from(self.game.cols * SQUARE_SIZE) - time_rect_width - MARGIN,
                    MARGIN,
                    time_rect_width,
                    UI_RECT_HEIGHT,
                ],
                &Default::default(),
                c.transform,
                g,
            );

            // hard coded 2 pixel offset
            let board_transform = c.transform.trans(2.0, 2.0 + f64::from(TOP_BAR_HEIGHT));

            let face_width = f64::from(icons.ongoing_face.get_width()) * FACE_ICON_SCALE;
            let face_height = f64::from(icons.ongoing_face.get_height()) * FACE_ICON_SCALE;
            let face_x = f64::from(self.game.cols * SQUARE_SIZE) * 0.5 - face_width * 0.5;
            let face_y = f64::from(TOP_BAR_HEIGHT) * 0.5 - face_height * 0.5;

            let face_transform = c.transform.trans(face_x, face_y).zoom(FACE_ICON_SCALE);

            // draw face button
            self.draw_face_button(&c, g, face_x, face_y, face_width, face_height);

            // render all triangles first in batch
            for i in 0..self.game.rows {
                for j in 0..self.game.cols {
                    let curr_x = j * SQUARE_SIZE;
                    let curr_y = i * SQUARE_SIZE;

                    let curr_square = self.game.get_square(i, j);

                    match curr_square.state {
                        SquareState::Covered => {
                            let color = if self.left_mouse_pressed
                                && self.selected_position.is_some()
                                && Position(i, j) == self.selected_position.unwrap()
                            {
                                CELL_REVEALED_COLOR
                            } else {
                                CELL_COVERED_COLOR
                            };

                            rectangle::Rectangle::new_border(CELL_BORDER_COLOR, 1.0)
                                .color(color)
                                .draw(
                                    [
                                        f64::from(curr_x),
                                        f64::from(curr_y),
                                        f64::from(SQUARE_SIZE) - 4.0,
                                        f64::from(SQUARE_SIZE) - 4.0,
                                    ],
                                    &Default::default(),
                                    board_transform,
                                    g,
                                );
                        }
                        SquareState::Revealed => {
                            let rect = if curr_square.is_mine {
                                rectangle::Rectangle::new(MINE_BORDER_COLOR).border(Border {
                                    color: MINE_REVEALED_COLOR,
                                    radius: 1.0,
                                })
                            } else {
                                rectangle::Rectangle::new(CELL_BORDER_COLOR).border(Border {
                                    color: CELL_REVEALED_COLOR,
                                    radius: 1.0,
                                })
                            };

                            rect.draw(
                                [
                                    f64::from(curr_x),
                                    f64::from(curr_y),
                                    f64::from(SQUARE_SIZE) - 4.0,
                                    f64::from(SQUARE_SIZE) - 4.0,
                                ],
                                &Default::default(),
                                board_transform,
                                g,
                            );
                        }
                        SquareState::Flagged => {
                            rectangle::Rectangle::new_border(CELL_BORDER_COLOR, 1.0)
                                .color(CELL_COVERED_COLOR)
                                .draw(
                                    [
                                        f64::from(curr_x),
                                        f64::from(curr_y),
                                        f64::from(SQUARE_SIZE) - 4.0,
                                        f64::from(SQUARE_SIZE) - 4.0,
                                    ],
                                    &Default::default(),
                                    board_transform,
                                    g,
                                );
                        }
                    }
                }
            }

            // then render all texts and images in batch
            for i in 0..self.game.rows {
                for j in 0..self.game.cols {
                    let curr_x = j * SQUARE_SIZE;
                    let curr_y = i * SQUARE_SIZE;

                    let curr_square = self.game.get_square(i, j);

                    let text_transform = board_transform
                        .trans(
                            f64::from(curr_x) + f64::from(SQUARE_SIZE) * 0.19,
                            f64::from(curr_y) + f64::from(SQUARE_SIZE) * 0.65,
                        )
                        .zoom(0.5);

                    let mine_transform = board_transform
                        .trans(
                            f64::from(curr_x) + f64::from(SQUARE_SIZE) * 0.06,
                            f64::from(curr_y) + f64::from(SQUARE_SIZE) * 0.06,
                        )
                        .zoom(0.07);

                    match curr_square.state {
                        SquareState::Revealed => {
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
                            let flag_transform = board_transform
                                .trans(
                                    f64::from(curr_x) + f64::from(SQUARE_SIZE) * 0.085,
                                    f64::from(curr_y) + f64::from(SQUARE_SIZE) * 0.085,
                                )
                                .zoom(0.10);

                            image(&icons.flag, flag_transform, g);
                        }
                        _ => (),
                    }
                }
            }

            let flag_num_transform = c.transform.trans(3.5, UI_FONT_Y_OFFSET).zoom(0.5);

            text(
                UI_TEXT_COLOR,
                UI_FONT_SIZE,
                &format!("{:03}", self.game.get_flags_left()),
                glyphs,
                flag_num_transform,
                g,
            ).unwrap();

            match self.game.state {
                GameState::Ongoing => image(&icons.ongoing_face, face_transform, g),
                GameState::Won => image(&icons.win_face, face_transform, g),
                GameState::Lost => image(&icons.lost_face, face_transform, g),
            }

            let time_transform = c.transform
                .trans(
                    f64::from(self.game.cols * SQUARE_SIZE) - 60.0,
                    UI_FONT_Y_OFFSET,
                )
                .zoom(0.5);

            text(
                UI_TEXT_COLOR,
                UI_FONT_SIZE,
                &format!("{:04}", self.game.game_time()),
                glyphs,
                time_transform,
                g,
            ).unwrap();
        });
    }
}

extern crate piston_window;
extern crate rand;
#[macro_use]
extern crate maplit;
extern crate clap;
extern crate find_folder;

mod frontend;
mod game;

use clap::App;
use piston_window::*;
use std::cmp::min;

fn main() {
    let app = App::new("minesweep-rs")
        .version("0.1.0")
        .author("Eric Fang")
        .about("Clone of Windows Minesweeper written in Rust")
        .args_from_usage(
            "-d, --difficulty=[LEVEL]  'Preset Difficulty Level, 1=Beginner 2=Intermediate 3=Expert 4=Custom'
                    -r, --rows=[ROWS]         'Sets number of rows (Custom level only)'
                    -c, --cols=[COLS]         'Sets number of columns (Custom level only)'
                    -m, --mines=[MINES]       'Sets max number of mines (Custom level only)'");

    let matches = app.get_matches();

    let difficulty = matches.value_of("difficulty").unwrap_or("1");

    let rows = matches
        .value_of("rows")
        .unwrap_or("15")
        .parse::<u32>()
        .unwrap();
    let cols = matches
        .value_of("cols")
        .unwrap_or("12")
        .parse::<u32>()
        .unwrap();
    let max_mines = matches
        .value_of("mines")
        .unwrap_or("13")
        .parse::<u32>()
        .unwrap();

    let mut front = match difficulty {
        "1" => frontend::Gui::new(8, 8, 10),
        "2" => frontend::Gui::new(16, 16, 40),
        "3" => frontend::Gui::new(24, 24, 99),
        "4" => frontend::Gui::new(cols, rows, min(max_mines, rows * cols)),
        _ => panic!("invalid difficulty level!"),
    };

    let mut window: PistonWindow = WindowSettings::new("Mine Sweeper", front.get_window_size())
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let font = &assets.join("Andale-Mono.ttf");
    let mine = &assets.join("mine.png");
    let flag = &assets.join("flag.png");
    let win_face = &assets.join("cool.png");
    let ongoing_face = &assets.join("happy.png");
    let lost_face = &assets.join("shocked.png");

    let factory = window.factory.clone();

    let mut glyphs = Glyphs::new(
        font,
        factory,
        TextureSettings::new().filter(Filter::Nearest),
    ).unwrap();

    let icons = frontend::Icons {
        mine: Texture::from_path(
            &mut window.factory,
            &mine,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap(),
        flag: Texture::from_path(
            &mut window.factory,
            &flag,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap(),
        win_face: Texture::from_path(
            &mut window.factory,
            &win_face,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap(),
        ongoing_face: Texture::from_path(
            &mut window.factory,
            &ongoing_face,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap(),
        lost_face: Texture::from_path(
            &mut window.factory,
            &lost_face,
            Flip::None,
            &TextureSettings::new(),
        ).unwrap(),
    };

    window.set_max_fps(30);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            front.draw(&mut window, &e, &mut glyphs, &icons);
        }

        if let Some(mouse_e) = e.mouse_cursor_args() {
            front.handle_mouse_position(mouse_e[0], mouse_e[1]);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(k) => front.handle_key_press(k),
                _ => (),
            }
        }

        if let Some(button) = e.release_args() {
            match button {
                Button::Mouse(m) => front.handle_mouse_click(m),
                _ => (),
            }
        }
    }
}

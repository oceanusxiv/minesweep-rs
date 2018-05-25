extern crate piston_window;
extern crate rand;
#[macro_use]
extern crate maplit;
extern crate find_folder;

mod frontend;
mod game;

use piston_window::*;

fn main() {
    println!("Hello, world!");

    let mut front = frontend::Gui::new();

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

    let mine: G2dTexture = Texture::from_path(
        &mut window.factory,
        &mine,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let flag: G2dTexture = Texture::from_path(
        &mut window.factory,
        &flag,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let win_face: G2dTexture = Texture::from_path(
        &mut window.factory,
        &win_face,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let ongoing_face: G2dTexture = Texture::from_path(
        &mut window.factory,
        &ongoing_face,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let lost_face: G2dTexture = Texture::from_path(
        &mut window.factory,
        &lost_face,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    window.set_max_fps(30);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            front.draw(
                &mut window,
                &e,
                &mut glyphs,
                &mine,
                &flag,
                &win_face,
                &ongoing_face,
                &lost_face,
            );
        }

        if let Some(mouse_e) = e.mouse_cursor_args() {
            front.handle_mouse_position(mouse_e[0], mouse_e[1]);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(m) => front.handle_mouse_click(m),
                Button::Keyboard(k) => front.handle_key_press(k),
                _ => (),
            }
        }
    }
}

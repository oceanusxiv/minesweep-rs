extern crate piston_window;
extern crate prettytable;
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
    let ref font = assets.join("FiraSans-Regular.ttf");
    println!("{:?}", font);
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(
        font,
        factory,
        TextureSettings::new().filter(Filter::Nearest),
    ).unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            front.draw(&mut window, &e, &mut glyphs);
        }

        if let Some(mouse_e) = e.mouse_cursor_args() {
            front.handle_mouse_position(mouse_e[0], mouse_e[1]);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(m) => front.handle_mouse_click(m),
                Button::Keyboard(k) => println!("{:?}", k),
                Button::Controller(c) => println!("{:?}", c),
            }
        }
    }
}

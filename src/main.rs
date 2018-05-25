extern crate piston_window;
extern crate prettytable;
extern crate rand;
#[macro_use]
extern crate maplit;

mod frontend;
mod game;

use piston_window::*;

fn main() {
    println!("Hello, world!");

    let mut game = frontend::Gui::new();

    let mut window: PistonWindow = WindowSettings::new("Mine Sweeper", game.get_window_size())
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            game.draw(&mut window, &e);
        }

        if let Some(mouse_e) = e.mouse_cursor_args() {
            game.handle_mouse_position(mouse_e);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(m) => println!("{:?}", m),
                Button::Keyboard(k) => println!("{:?}", k),
                Button::Controller(c) => println!("{:?}", c),
            }
        }
    }
}

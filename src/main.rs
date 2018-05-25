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

    let game = game::MineSweeper::new(9, 9, 10);

    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        if let Some(mouse_e) = e.mouse_cursor_args() {
            println!("{}, {}", mouse_e[0], mouse_e[1]);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(m) => println!("{:?}", m),
                Button::Keyboard(k) => println!("{:?}", k),
                Button::Controller(c) => println!("{:?}", c),
            }
        }

        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],     // red
                [0.0, 0.0, 100.0, 100.0], // rectangle
                c.transform,
                g,
            );
        });
    }
}

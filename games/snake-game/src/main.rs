extern crate piston_window;

mod game;

use piston_window::*;
use piston_window::types::Color;



use game::Game;

const BACK_COLOR:Color = [0.207,0.286,0.47,1.0];
const WINDOW_W:u32 = 1024;
const WINDOW_H:u32 = 768;



fn main() {
    
    //create a window 
    let mut window: PistonWindow = WindowSettings::new("Hello Meow", [WINDOW_W,WINDOW_H])
        .exit_on_esc(true).build().unwrap();
    
    let mut game = Game::new(); 

    //event loop    
    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(key)) = event.press_args() {
          game.key_pressed(key);
        }

        window.draw_2d(&event,|c, g| {
            game.draw(&c,g);
        });
    }
}

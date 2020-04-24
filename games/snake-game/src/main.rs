extern crate piston_window;
use piston_window::*;
use piston_window::types::Color;


mod snake;
use snake::{Snake,Direction};


const BACK_COLOR:Color = [0.207,0.286,0.47,1.0];
const WINDOW_W:u32 = 1024;
const WINDOW_H:u32 = 768;

fn main() {
    
    //create a window 
    let mut window: PistonWindow = WindowSettings::new("Hello Meow", [WINDOW_W,WINDOW_H])
        .exit_on_esc(true).build().unwrap();
    
    let mut snake = Snake::new(750.0,500.0);

    //event loop    
    while let Some(event) = window.next() {

        //if let Some(Button::Mouse(button)) = event.press_args() {
        //    println!("{:?} fagsjdkflgkkh",button);
        //}

        if let Some(Button::Keyboard(button)) = event.press_args() {
            println!("{:?}",button);
            match button{
                Key::Up    => snake.move_forward(Some(Direction::Up)),
                Key::Down  => snake.move_forward(Some(Direction::Down)),
                Key::Left  => snake.move_forward(Some(Direction::Left)),
                Key::Right => snake.move_forward(Some(Direction::Right)),
                Key::Space => snake.move_forward(None),
                _          => {}
            }
        }

        window.draw_2d(&event,|c, g| {
            clear(BACK_COLOR,g);
            snake.draw(&c,g);
        });
    }
}

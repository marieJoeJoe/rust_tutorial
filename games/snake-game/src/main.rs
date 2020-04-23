extern crate piston_window;
use piston_window::*;
use piston_window::types::Color;


mod snake;
use snake::Snake;


const BACK_COLOR:Color = [0.207,0.286,0.47,1.0];

struct Point<T>{
    x:T,
    y:T
}

const window_w:u32 = 1024;
const window_h:u32 = 768;

fn main() {
    
    //create a window 
    let mut window: PistonWindow = WindowSettings::new("Hello Meow", [window_w,window_h])
        .exit_on_esc(true).build().unwrap();

    let mut snake_pos:Point<f64> = Point {x:200.0,y:200.0};
    
    let mut snake = Snake::new(750.0,500.0);

    //event loop    
    while let Some(event) = window.next() {

        //if let Some(Button::Mouse(button)) = event.press_args() {
        //    println!("{:?} fagsjdkflgkkh",button);
        //}

        if let Some(Button::Keyboard(button)) = event.press_args() {
            println!("{:?}",button);
            match button{
                //Key::Up    => {snake_pos.y -=10.0;},
                //Key::Down  => {snake_pos.y +=10.0;},
                //Key::Left  => {snake_pos.x -=10.0;},
                //Key::Right => {snake_pos.x +=10.0;},
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

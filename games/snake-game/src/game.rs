
use piston_window::*;
use piston_window::types::Color;


mod snake;
use snake::{Snake, Direction};


const FOOD_COLOR:Color = [0.90, 0.49 ,0.13, 1.0];
const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.5];


const BLOCK_SIZE: f64 = 50.0;


pub struct Game {

  snake: Snake,

  //food

  food_x: f64,
  food_y: f64,

}

impl Game {
  pub fn new() -> Game{
    Game{
      snake: Snake::new(100.0, 100.0),
      food_x: 550.0,
      food_y: 350.0,
    }
  }

  pub fn key_pressed(&mut self, key: Key){
    println!("{:?}",key);
    match key {
      Key::Up    => self.snake.move_forward(Some(Direction::Up)),
      Key::Down  => self.snake.move_forward(Some(Direction::Down)),
      Key::Left  => self.snake.move_forward(Some(Direction::Left)),
      Key::Right => self.snake.move_forward(Some(Direction::Right)),
      Key::Space => self.snake.move_forward(None),
      _          => {}
    }

    self.check_eating();
 
  }
  
  pub fn draw(&self, con: &Context, g :&mut G2d){
    self.snake.draw(&con,g);

    rectangle(FOOD_COLOR,[self.food_x,self.food_y,BLOCK_SIZE,BLOCK_SIZE],con.transform,g);
  }

  pub fn check_eating(&mut self){
  
  }


}

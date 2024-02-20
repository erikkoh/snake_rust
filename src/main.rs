use std::time::{Duration, Instant};
use std::vec;

use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect, Text, TextFragment};
use ggez::event::{self, EventHandler};
use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::{thread_rng, Error, Rng};


const GIRD_DIMENSION: (f32,f32) = (20.0,20.0);
const CELL_SIZE: f32 = 30.0;


//kodet under b8 xxv11 Orange Orangutan






fn main() ->GameResult{
    // Make a Context.
    let window_size = (CELL_SIZE*GIRD_DIMENSION.0, CELL_SIZE*GIRD_DIMENSION.1); 
    let window_mode = conf::WindowMode::default().dimensions(window_size.0, window_size.1).resizable(false);
    let window_setup = conf::WindowSetup::default().title("Snakel").vsync(true);
    let (mut ctx, event_loop) = ContextBuilder::new("snake", "Erikkoh").window_mode(window_mode).window_setup(window_setup)
        .build()
        .expect("Its joeover!");


    let  state = Mainstate::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, state);

}
struct Grid{
}

impl  Grid{
    fn gridposition(pos: f32) -> f32 {
        let result = pos*CELL_SIZE;
        result
    }
    fn get_grid()->Vec<Vec<f32>>{
        let mut grid: Vec<Vec<f32>> = vec![];
        for i in 1..(GIRD_DIMENSION.0 as u32-1){
            for j in 1..(GIRD_DIMENSION.1 as u32-1){
                grid.push(vec![i as f32,j as f32]);
            }
        }
        grid
    }

}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}



impl Direction {
    fn convert_dir(dir: &Direction) -> Vec<f32> {
        match dir {
            Direction::Up => vec![0.0, -1.0],
            Direction::Down => vec![0.0, 1.0],
            Direction::Left => vec![-1.0, 0.0],
            Direction::Right => vec![1.0, 0.0],
        }
    }
    fn check_direction(dir: &Direction, keypress: ggez::input::keyboard::KeyCode) -> bool{
        match (dir, keypress){
            (Direction::Up, ggez::input::keyboard::KeyCode::S) => false,
            (Direction::Down, ggez::input::keyboard::KeyCode::W) => false,
            (Direction::Left, ggez::input::keyboard::KeyCode::D) => false,
            (Direction::Right, ggez::input::keyboard::KeyCode::A) => false,
            _ => {
                true
            }
        }
    }
}




struct Snake{
    snake_mesh: Vec<Mesh>,
    snake_array: Vec<Vec<f32>>,
    snake_direction: Direction,

}

impl Snake{
    fn new(ctx: &mut Context) -> GameResult<Self>{
        let snake_array = vec![vec![GIRD_DIMENSION.0*1.0/2.0-1.0,GIRD_DIMENSION.1*1.0/2.0-1.0],vec![GIRD_DIMENSION.1*1.0/2.0-2.0,GIRD_DIMENSION.1*1.0/2.0-1.0]];
        let snake_mesh = vec![graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(0.0,0.0, CELL_SIZE, CELL_SIZE), graphics::Color::BLUE)?,
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(0.0,0.0, CELL_SIZE, CELL_SIZE), graphics::Color::WHITE)?,
        ];
        let snake_direction = Direction:: Right;
        Ok(Self{
            snake_array, 
            snake_mesh,
            snake_direction,
        })

    }

    fn eat_food(&mut self, ctx: &mut Context)->GameResult{
        let tail:Vec<f32> = self.snake_array[self.snake_array.len()-1].clone();
        self.move_snake();       
        let new_tail_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new( 0.0, 0.0, CELL_SIZE, CELL_SIZE), graphics::Color::WHITE)?;
        self.snake_array.push(tail); 
        self.snake_mesh.push(new_tail_mesh);

        Ok(())
        }

    fn move_snake(&mut self){
        let movment = Direction:: convert_dir(&self.snake_direction);
        for i in (1..=self.snake_array.len()-1).rev(){
            let new_pos:Vec<f32> = self.snake_array[i-1].clone();
            self.snake_array[i] = new_pos;
        }
        self.snake_array[0] = vec![self.snake_array[0][0]+movment[0],self.snake_array[0][1]+movment[1]];
        

    }

    fn draw(&self, canvas:&mut graphics::Canvas){
        for i in 0..=(self.snake_array.len()-1) {
            canvas.draw(&self.snake_mesh[i], Vec2::new(Grid::gridposition(self.snake_array[i][0]),Grid::gridposition(self.snake_array[i][1])));
        }
    }

}

struct Food{
    food_mesh: Mesh,
    food_pos:Vec<f32>,


}

impl Food {
    fn new(ctx: &mut Context)->GameResult<Self>{
        let rng = thread_rng();
        let food_pos = vec![(GIRD_DIMENSION.0)/2.0+(GIRD_DIMENSION.0)/5.0,((GIRD_DIMENSION.1)/2.0)];
        let food_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), graphics::Rect::new(0.0,0.0,CELL_SIZE,CELL_SIZE), graphics::Color::RED)?;
        Ok(Self{
            food_mesh,
            food_pos,
        })
    }
    fn new_position(&mut self, snake_array: &Vec<Vec<f32>>, board: &Vec<Vec<f32>>){
        let mut rng = thread_rng();
        let mut new_board = board.clone();
        for i in 0..snake_array.len(){
            for j in 0..new_board.len(){
                if new_board[j] == snake_array[i]{
                    new_board.remove(j);
                    break;
                }
            }    
        }
        let new_pos: Vec<f32> = new_board[rng.gen_range(0..new_board.len())].clone();
        self.food_pos = new_pos;


    }
    fn draw(&mut self, canvas:&mut graphics::Canvas){
        canvas.draw(&self.food_mesh,Vec2::new(Grid::gridposition(self.food_pos[0]),Grid::gridposition(self.food_pos[1])));
    }
    
}



struct GameState{
    game_over: bool,
    food_count: i32,
    game_start: bool,


}

impl GameState{
    fn new() -> GameState{
        let game_over = false;
        let game_start = false;
        let food_count = 0;


        GameState{
            game_over,
            food_count,
            game_start,
        }
    }

    fn check_game_state(&mut self, pos_array: Vec<Vec<f32>>){
        let head = pos_array[0].clone();
        for i in  1..=(pos_array.len()-1){
            if head == pos_array[i]{
                self.game_over = true;
                break;
            }

            }
        if head.contains(&0.0) || head.contains(&(GIRD_DIMENSION.0-1.0)){
            self.game_over = true;        
        }

    } 
}


struct Button{
    button_bouds: Vec<Vec<f32>>,
    button_render: bool,
    button_image: Image,
    button_size: f32,
}

impl Button{
    fn new(size: f32, pos: Vec<f32>, button_type: &str, ctx: &mut Context ) -> GameResult<Self>{
        let button_bouds = vec![vec![pos[0], pos[1]], vec![pos[1] + size, pos [1] + size]];
        let button_render = true;
        let button_size = size;
        let button_image: Image = graphics::Image::from_path(ctx, "/playbutton.png")?;
        // let button_image: Mesh =  graphics::Mesh::new_rectangle(ctx, DrawMode::fill(),Rect::new(0.0, 0.0, 60.0, 60.0) ,graphics::Color::RED )?;        
        
        // match button_type{
        //     "playbutton" => {
        //         let button_image = graphics::Image::from_path(ctx, "/playbutton.png")?;  
        //     }
        //     _ => {
        //         println!("not a valid button type")
        //     }
        // }


        Ok(Self{
            button_bouds,
            button_render,
            button_image,
            button_size,
        })
    }


    fn draw(&mut self, canvas: &mut graphics::Canvas){
        if self.button_render {
            canvas.draw(&self.button_image, DrawParam::new().dest(Vec2::new(Grid::gridposition(self.button_bouds[0][0]),Grid::gridposition(self.button_bouds[0][1]))).z(1).scale([self.button_size,self.button_size]));
    
        }
    }
}

struct Mainstate {
    snake: Snake,
    movment: Instant,
    food: Food,
    border: Mesh,
    game_state: GameState,
    valid_direction: Vec<Direction>,
    board: Vec<Vec<f32>>,
    start_button: Button,

}


impl Mainstate{
    fn new(ctx: &mut Context) -> GameResult<Mainstate>{
        let snake = Snake::new(ctx)?;
        let movment: Instant = Instant::now(); 
        let food = Food::new(ctx)?;
        let border = graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(CELL_SIZE*2.0), Rect::new(0.0, 0.0, GIRD_DIMENSION.0*CELL_SIZE, GIRD_DIMENSION.1*CELL_SIZE), graphics::Color::GREEN)?;
        let game_state = GameState::new();
        let valid_direction = vec![snake.snake_direction];
        let board = Grid::get_grid();
        let start_button = Button::new(4.0, vec![10.0,10.0], "playbutton", ctx)?;

        Ok(Mainstate{
            food,
            snake,
            movment,
            border,
            game_state,
            valid_direction,
            board,
            start_button,
        })
    }   
}

impl EventHandler for  Mainstate{
    
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.game_state.game_over{
            let check:Option<Duration> = Instant::now().checked_duration_since(self.movment); //should be replaced by  fn check_update_time(&mut self, target_fps: u32) -> bool
            match check{
                Some(duration) => {
                    if duration > Duration::from_millis(200){
                        if self.valid_direction.len() > 1 {
                            self.snake.snake_direction = self.valid_direction[1];
                            self.valid_direction.remove(0);
                        }
                        self.snake.move_snake();
                        self.movment = Instant::now();
                        if self.snake.snake_array[0] == self.food.food_pos{
                            self.snake.eat_food(_ctx,)?;
                            self.game_state.food_count +=1;
                            self.food.new_position(&self.snake.snake_array, &self.board);
                        };
                }  
                 
            }
            None => {
            }
        }
        self.game_state.check_game_state(self.snake.snake_array.clone());
    }
         Ok(())
     }
 


     fn draw(&mut self, ctx: &mut Context) -> GameResult {
         let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        
        self.start_button.draw(&mut canvas);

        self.snake.draw(&mut canvas);


        canvas.draw(&self.border, DrawParam::default());
        canvas.draw(graphics::Text::new((self.game_state.food_count).to_string()).set_scale(24.0),Vec2::new(0.0,0.0));
        self.food.draw(&mut canvas);
        canvas.finish(ctx)?;
        Ok(())
     }
    
    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> GameResult{
            let bottum_value = vec![Grid::gridposition(self.start_button.button_bouds[0][0]),Grid::gridposition(self.start_button.button_bouds[0][1])];
            let top_value = vec![Grid::gridposition(self.start_button.button_bouds[1][0]),Grid::gridposition(self.start_button.button_bouds[1][1])];
            let click = vec![_x,_y];
            if bottum_value <= click && click <= top_value{
                self.game_state.game_start= true;
            }
            Ok(())
        } 

     fn key_down_event(
             &mut self,
             _ctx: &mut Context,
             input: KeyInput,
             _repeated: bool,
         ) -> GameResult {
            let last_element = self.valid_direction.len();
         match input.keycode{
            Some(KeyCode::W) => {
                if Direction::check_direction(&self.valid_direction[last_element-1], KeyCode::W){
                self.valid_direction.push(Direction::Up);
                }
            },
            Some(KeyCode::S) => {
                if Direction::check_direction(&self.valid_direction[last_element-1], KeyCode::S){
                    self.valid_direction.push(Direction::Down);

                }
                },
            Some(KeyCode::A) =>{
                if Direction::check_direction(&&self.valid_direction[last_element-1], KeyCode::A){
                    self.valid_direction.push(Direction::Left);        
                }
                },
            Some(KeyCode::D) =>{
                if Direction::check_direction(&self.valid_direction[last_element-1], KeyCode::D){
                    self.valid_direction.push( Direction::Right);
                }
                },
            _ => (),

         }
        Ok(())

     }


    }
 

 
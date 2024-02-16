use std::time::{Duration, Instant};
use std::vec;

use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect, Text, TextFragment};
use ggez::event::{self, EventHandler};
use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::{thread_rng, Rng};


const GIRD_DIMENSION: (f32,f32) = (10.0,10.0);
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
        for i in 1..(GIRD_DIMENSION.0 as u32){
            for j in 1..=(GIRD_DIMENSION.1 as u32-2){
                grid.push(vec![i as f32,j as f32]);
            }
        }
        grid
    }

}

#[derive(Copy, Clone)]
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
        //optimze
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


}

impl GameState{
    fn new() -> GameState{
        let game_over = false;
        let food_count = 0;

        GameState{
            game_over,
            food_count,
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
    button_cliked: bool,
    button_render: bool,

}

impl Button{
    fn new(size: f32, pos: Vec<f32>) -> GameResult<Button>{
        let button_bouds = vec![vec![pos[0], pos[1]], vec![pos[1] + size, pos [1] + size]];
        let button_cliked = false;
        let button_render = false;
        Ok((Button{
            button_bouds,
            button_cliked,
            button_render,
        }))
    }
    fn new_play_button_mesh(ctx: &mut Context)->GameResult<Image>{
        let button_image = graphics::Image::from_path(ctx, "/playbutton.png")?;
        Ok(button_image)

    }

    fn draw(&mut self, button_image: Image, ctx: &mut Context, mut canvas:Canvas){
        canvas.draw(&button_image, Vec2::new(Grid::gridposition(self.button_bouds[0][0]),Grid::gridposition(self.button_bouds[0][1])));
    }
}

struct Mainstate {
    snake: Snake,
    movment: Instant,
    food: Food,
    border: Mesh,
    game_over: GameState,
    valid_direction: Direction,
    board: Vec<Vec<f32>>,

}


impl Mainstate{
    fn new(ctx: &mut Context) -> GameResult<Mainstate>{
        let snake = Snake::new(ctx)?;
        let movment: Instant = Instant::now(); 
        let food = Food::new(ctx)?;
        let border = graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(CELL_SIZE*2.0), Rect::new(0.0, 0.0, GIRD_DIMENSION.0*CELL_SIZE, GIRD_DIMENSION.1*CELL_SIZE), graphics::Color::GREEN)?;
        let game_over = GameState::new();
        let valid_direction = snake.snake_direction;
        let board = Grid::get_grid();

        Ok(Mainstate{
            food,
            snake,
            movment,
            border,
            game_over,
            valid_direction,
            board,
        })
    }   
}

impl EventHandler for  Mainstate{
    
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.game_over.game_over{
            let check:Option<Duration> = Instant::now().checked_duration_since(self.movment); //should be replaced by  fn check_update_time(&mut self, target_fps: u32) -> bool
            match check{
                Some(duration) => {
                    if duration > Duration::from_millis(200){
                        self.snake.snake_direction = self.valid_direction;
                        self.snake.move_snake();
                        self.movment = Instant::now();
                        if self.snake.snake_array[0] == self.food.food_pos{
                            self.snake.eat_food(_ctx,)?;
                            self.game_over.food_count +=1;
                            self.food.new_position(&self.snake.snake_array, &self.board);
                        };
                }  
                 
            }
            None => {
            }
        }
        self.game_over.check_game_state(self.snake.snake_array.clone());
    }
         Ok(())
     }
 
     fn draw(&mut self, ctx: &mut Context) -> GameResult {
         let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        
        self.snake.draw(&mut canvas);

        canvas.draw(&self.border, DrawParam::default());
        canvas.draw(graphics::Text::new((self.game_over.food_count).to_string()).set_scale(24.0),Vec2::new(0.0,0.0));
        self.food.draw(&mut canvas);
        canvas.finish(ctx)?;
        Ok(())
     }
    
     fn key_down_event(
             &mut self,
             _ctx: &mut Context,
             input: KeyInput,
             _repeated: bool,
         ) -> GameResult {
         match input.keycode{
            Some(KeyCode::W) => {
                if Direction::check_direction(&self.snake.snake_direction, KeyCode::W){
                self.valid_direction = Direction:: Up}
            },
            Some(KeyCode::S) => {
                if Direction::check_direction(&self.snake.snake_direction, KeyCode::S){
                    self.valid_direction = Direction:: Down}
                },
            Some(KeyCode::A) =>{
                if Direction::check_direction(&self.snake.snake_direction, KeyCode::A){
                    self.valid_direction = Direction::Left}
                },
            Some(KeyCode::D) =>{
                if Direction::check_direction(&self.snake.snake_direction, KeyCode::D){
                    self.valid_direction = Direction:: Right}
                },
            _ => (),
         }
         Ok(())
     }
}
 

 
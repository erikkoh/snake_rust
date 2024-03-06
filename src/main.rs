use std::env;
use std::path;
use std::vec;
extern crate good_web_game as ggez;

use ggez::cgmath::Point2;
use ggez::event::{self, EventHandler};
use ggez::graphics::{
    self, Color, DrawMode, DrawParam, Mesh, Rect,
};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::timer::check_update_time;
use ggez::{ Context, GameResult};


const GIRD_DIMENSION: (f32, f32) = (20.0, 20.0);
const CELL_SIZE: f32 = 30.0;

//kodet under b8 xxv11 Orange Orangutan

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let conf = ggez::conf::Conf::default()
        .physical_root_dir(Some(resource_dir))
        .window_height((GIRD_DIMENSION.0 * CELL_SIZE) as i32)
        .window_width((GIRD_DIMENSION.1 * CELL_SIZE) as i32);

    ggez::start(conf, |mut context, mut _quad_ctx| {
        Box::new(Mainstate::new(&mut context, _quad_ctx).unwrap())
    })
}
struct Grid {}

impl Grid {
    fn gridposition(pos: f32) -> f32 {
        let result = pos * CELL_SIZE;
        result
    }
    fn get_grid() -> Vec<Vec<f32>> {
        let mut grid: Vec<Vec<f32>> = vec![];
        for i in 1..(GIRD_DIMENSION.0 as u32 - 1) {
            for j in 1..(GIRD_DIMENSION.1 as u32 - 1) {
                grid.push(vec![i as f32, j as f32]);
            }
        }
        grid
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
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
    fn check_direction(dir: &Direction, keypress: ggez::input::keyboard::KeyCode) -> bool {
        match (dir, keypress) {
            (
                Direction::Up,
                ggez::input::keyboard::KeyCode::S | ggez::input::keyboard::KeyCode::W,
            ) => false,
            (
                Direction::Down,
                ggez::input::keyboard::KeyCode::W | ggez::input::keyboard::KeyCode::S,
            ) => false,
            (
                Direction::Left,
                ggez::input::keyboard::KeyCode::D | ggez::input::keyboard::KeyCode::A,
            ) => false,
            (
                Direction::Right,
                ggez::input::keyboard::KeyCode::A | ggez::input::keyboard::KeyCode::D,
            ) => false,
            _ => true,
        }
    }
}

struct Snake {
    snake_mesh: Vec<Mesh>,
    snake_array: Vec<Vec<f32>>,
    snake_direction: Direction,
}

impl Snake {
    fn new(ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) -> GameResult<Self> {
        let snake_array = vec![
            vec![
                GIRD_DIMENSION.0 * 1.0 / 2.0 - 1.0,
                GIRD_DIMENSION.1 * 1.0 / 2.0 - 1.0,
            ],
            vec![
                GIRD_DIMENSION.1 * 1.0 / 2.0 - 2.0,
                GIRD_DIMENSION.1 * 1.0 / 2.0 - 1.0,
            ],
        ];
        let snake_mesh = vec![
            graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
                graphics::Color::BLUE,
            )?,
            graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
                graphics::Color::WHITE,
            )?,
        ];
        let snake_direction = Direction::Right;
        Ok(Self {
            snake_array,
            snake_mesh,
            snake_direction,
        })
    }

    fn eat_food(&mut self, ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) -> GameResult {
        let tail: Vec<f32> = self.snake_array[self.snake_array.len() - 1].clone();
        self.move_snake();
        let new_tail_mesh = graphics::Mesh::new_rectangle(
            ctx,
            quad_ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
            graphics::Color::WHITE,
        )?;
        self.snake_array.push(tail);
        self.snake_mesh.push(new_tail_mesh);

        Ok(())
    }

    fn move_snake(&mut self) {
        let movment = Direction::convert_dir(&self.snake_direction);
        for i in (1..=self.snake_array.len() - 1).rev() {
            let new_pos: Vec<f32> = self.snake_array[i - 1].clone();
            self.snake_array[i] = new_pos;
        }
        self.snake_array[0] = vec![
            self.snake_array[0][0] + movment[0],
            self.snake_array[0][1] + movment[1],
        ];
    }

    fn draw(&self, ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) {
        for i in 0..=(self.snake_array.len() - 1) {
            graphics::draw(
                ctx,
                quad_ctx,
                &self.snake_mesh[i],
                DrawParam::new().dest(Point2::new(
                    Grid::gridposition(self.snake_array[i][0]),
                    Grid::gridposition(self.snake_array[i][1]),
                )),
            );
        }
    }

    fn reset(&mut self, ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) -> GameResult {
        self.snake_array = vec![
            vec![
                GIRD_DIMENSION.0 * 1.0 / 2.0 - 1.0,
                GIRD_DIMENSION.1 * 1.0 / 2.0 - 1.0,
            ],
            vec![
                GIRD_DIMENSION.1 * 1.0 / 2.0 - 2.0,
                GIRD_DIMENSION.1 * 1.0 / 2.0 - 1.0,
            ],
        ];

        self.snake_direction = Direction::Right;
        self.snake_mesh = vec![
            graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
                graphics::Color::BLUE,
            )?,
            graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
                graphics::Color::WHITE,
            )?,
        ];
        Ok(())
    }
}

struct Food {
    food_mesh: Mesh,
    food_pos: Vec<f32>,
}

impl Food {
    fn new(ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) -> GameResult<Self> {
        let food_pos = vec![
            (GIRD_DIMENSION.0) / 2.0 + (GIRD_DIMENSION.0) / 5.0,
            ((GIRD_DIMENSION.1) / 2.0),
        ];
        let food_mesh = graphics::Mesh::new_rectangle(
            ctx,
            quad_ctx,
            DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
            graphics::Color::RED,
        )?;
        Ok(Self {
            food_mesh,
            food_pos,
        })
    }
    fn new_position(&mut self, snake_array: &Vec<Vec<f32>>, board: &Vec<Vec<f32>>) {
        let mut new_board = board.clone();
        for i in 0..snake_array.len() {
            for j in 0..new_board.len() {
                if new_board[j] == snake_array[i] {
                    new_board.remove(j);
                    break;
                }
            }
        }
        let new_pos: Vec<f32> = new_board[quad_rand::gen_range(0, new_board.len())].clone();
        self.food_pos = new_pos;
    }
    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) {
        graphics::draw(
            ctx,
            quad_ctx,
            &self.food_mesh,
            DrawParam::new().dest(Point2::new(
                Grid::gridposition(self.food_pos[0]),
                Grid::gridposition(self.food_pos[1]),
            )),
        );
    }

    fn reset(&mut self) {
        self.food_pos = vec![
            (GIRD_DIMENSION.0) / 2.0 + (GIRD_DIMENSION.0) / 5.0,
            ((GIRD_DIMENSION.1) / 2.0),
        ];
    }
}

struct GameState {
    game_over: bool,
    food_count: i32,
    game_start: bool,
}

impl GameState {
    fn new() -> GameState {
        let game_over = false;
        let game_start = false;
        let food_count = 0;

        GameState {
            game_over,
            food_count,
            game_start,
        }
    }

    fn check_game_state(&mut self, pos_array: Vec<Vec<f32>>) {
        let head = pos_array[0].clone();
        for i in 1..=(pos_array.len() - 1) {
            if head == pos_array[i] {
                self.game_over = true;
                break;
            }
        }
        if head.contains(&0.0) || head.contains(&(GIRD_DIMENSION.0 - 1.0)) {
            self.game_over = true;
        }
    }
    fn reset(&mut self) {
        self.game_over = false;
        self.food_count = 0;
    }
}

// struct Button {
//     button_bouds: Vec<Vec<f32>>,
//     button_render: bool,
//     button_image: Image,
// }

// impl Button {
//     fn new(
//         size: f32,
//         pos: Vec<f32>,
//         button_type: &str,
//         ctx: &mut Context,
//         quad_ctx: &mut event::GraphicsContext,
//     ) -> GameResult<Self> {
//         let button_bouds = vec![vec![pos[0], pos[1]], vec![pos[0] + size, pos[1] + size]];
//         let button_render = true;
//         let button_image: Image = graphics::Image::new(ctx, quad_ctx, "/playbutton.png")?;

//         Ok(Self {
//             button_bouds,
//             button_render,
//             button_image,
//         })
//     }

//     fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) {
//         if self.button_render {
//             graphics::draw(
//                 ctx,
//                 quad_ctx,
//                 &self.button_image,
//                 DrawParam::new()
//                     .dest(Point2::new(
//                         Grid::gridposition(self.button_bouds[0][0]),
//                         Grid::gridposition(self.button_bouds[0][1]),
//                     ))
//                     .scale([
//                         CELL_SIZE / 30.0 * (self.button_bouds[1][0] - self.button_bouds[0][0]),
//                         CELL_SIZE / 30.0 * (self.button_bouds[1][1] - self.button_bouds[0][1]),
//                     ]),
//             );
//         }
//     }
// }

struct Mainstate {
    snake: Snake,
    // movment: Instant,
    food: Food,
    border: Mesh,
    game_state: GameState,
    valid_direction: Vec<Direction>,
    board: Vec<Vec<f32>>,
    // start_button: Button,
}

impl Mainstate {
    fn new(ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) -> GameResult<Mainstate> {
        let snake = Snake::new(ctx, quad_ctx)?;
        // let movment: Instant = Instant::now();
        let food = Food::new(ctx, quad_ctx)?;
        let border = graphics::Mesh::new_rectangle(
            ctx,
            quad_ctx,
            DrawMode::stroke(CELL_SIZE),
            Rect::new(
                CELL_SIZE/2.0,
                CELL_SIZE/2.0,
                (GIRD_DIMENSION.0 - 1.0 )* CELL_SIZE,
                (GIRD_DIMENSION.1 - 1.0)* CELL_SIZE,
            ),
            graphics::Color::GREEN,
        )?;
        let game_state = GameState::new();
        let valid_direction = vec![snake.snake_direction];
        let board = Grid::get_grid();
        let start_button_size = 8.0;
        let start_button_pos = vec![
            (GIRD_DIMENSION.0 - start_button_size) / 2.0,
            (GIRD_DIMENSION.1 - start_button_size) / 2.0,
        ];
        // let start_button = Button::new(
        //     start_button_size,
        //     start_button_pos,
        //     "playbutton",
        //     ctx,
        //     quad_ctx,
        // )?;

        Ok(Mainstate {
            food,
            snake,
            // movment,
            border,
            game_state,
            valid_direction,
            board,
            // start_button,
        })
    }
    fn reset(&mut self, ctx: &mut Context, quad_ctx: &mut event::GraphicsContext) {
        self.snake.reset(ctx, quad_ctx);
        self.food.reset();
        self.valid_direction = vec![self.snake.snake_direction];
        self.game_state.reset();
    }
}

impl EventHandler for Mainstate {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut event::GraphicsContext) -> GameResult {
        if check_update_time(_ctx, 10) {
        if !self.game_state.game_over && self.game_state.game_start {
            // let check: Option<Duration> = Instant::now().checked_duration_since(self.movment); //should be replaced by  fn check_update_time(&mut self, target_fps: u32) -> bool

                    // if duration > Duration::from_millis(50) {
                        if self.valid_direction.len() > 1 {
                            self.snake.snake_direction = self.valid_direction[1];
                            self.valid_direction.remove(0);
                        }
                        self.snake.move_snake();
                        // self.movment = Instant::now();
                        if self.snake.snake_array[0] == self.food.food_pos {
                            self.snake.eat_food(_ctx, _quad_ctx)?;
                            self.game_state.food_count += 1;
                            self.food.new_position(&self.snake.snake_array, &self.board);
                        // };
                    }
                }
            }
            self.game_state
                .check_game_state(self.snake.snake_array.clone());
            Ok(())
        }

    fn draw(&mut self, ctx: &mut Context, _quad_ctx: &mut event::GraphicsContext) -> GameResult {
        graphics::clear(ctx, _quad_ctx, Color::BLACK);

        // self.start_button.draw(ctx, _quad_ctx);

        self.snake.draw(ctx, _quad_ctx);

        graphics::draw(ctx, _quad_ctx, &self.border, DrawParam::default())?;
        graphics::draw(
            ctx,
            _quad_ctx,
            &graphics::Text::new("Score: ".to_owned() + (&(self.game_state.food_count).to_string())),
            DrawParam::default(),
        )?;
        self.food.draw(ctx, _quad_ctx);
        graphics::present(ctx, _quad_ctx)?;
        Ok(())
    }

    // fn mouse_button_down_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _quad_ctx: &mut event::GraphicsContext,
    //     _button: event::MouseButton,
    //     _x: f32,
    //     _y: f32,
    // ) {
    //     let bottum_value = vec![
    //         // Grid::gridposition(self.start_button.button_bouds[0][0]),
    //         // Grid::gridposition(self.start_button.button_bouds[0][1]),
    //     ];
    //     let top_value = vec![
    //         Grid::gridposition(self.start_button.button_bouds[1][0]),
    //         Grid::gridposition(self.start_button.button_bouds[1][1]),
    //     ];
    //     let click = vec![_x, _y];
    //     if bottum_value <= click && click <= top_value {
    //         self.game_state.game_start = true;
    //         self.start_button.button_render = false;
    //     }
    // }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut event::GraphicsContext,
        _keycod: KeyCode,
        _keymods: KeyMods,
        _repeated: bool,
    ) {
        self.game_state.game_start = true;
        let last_element = self.valid_direction.len();
        match _keycod {
            KeyCode::W | KeyCode::Up => {
                if Direction::check_direction(&self.valid_direction[last_element - 1], KeyCode::W) {
                    self.valid_direction.push(Direction::Up);
                }
            }
            KeyCode::S | KeyCode::Down => {
                if Direction::check_direction(&self.valid_direction[last_element - 1], KeyCode::S) {
                    self.valid_direction.push(Direction::Down);
                }
            }
            KeyCode::A | KeyCode::Left => {
                if Direction::check_direction(&&self.valid_direction[last_element - 1], KeyCode::A)
                {
                    self.valid_direction.push(Direction::Left);
                }
            }
            KeyCode::D | KeyCode::Right => {
                if Direction::check_direction(&self.valid_direction[last_element - 1], KeyCode::D) {
                    self.valid_direction.push(Direction::Right);
                }
            }
            KeyCode::R | KeyCode::Space => {
                self.reset(_ctx, _quad_ctx);
            }
            _ => (),
        }
    }
}

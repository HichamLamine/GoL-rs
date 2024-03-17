use std::cell;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    glam::vec2,
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text},
    mint::Point2,
    Context, ContextBuilder, GameResult,
};
use rand::{thread_rng, Rng};

const CELL_SIZE: f32 = 10.;
const GRID_WIDTH: f32 = 100.;
const GRID_HEIGHT: f32 = 100.;
const WINDOW_WIDTH: f32 = CELL_SIZE * GRID_WIDTH;
const WINDOW_HEIGHT: f32 = CELL_SIZE * GRID_HEIGHT;

enum State {
    Running,
    Pause,
}

struct GameState {
    grid: Vec<Vec<bool>>,
    generation: usize,
    target_fps: u32,
    state: State,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        ctx.gfx.set_window_title("Game-of-Life");
        Self {
            grid: vec![vec![false; GRID_HEIGHT as usize]; GRID_WIDTH as usize],
            generation: 0,
            target_fps: 20,
            state: State::Running,
        }
    }

    fn update_grid_state(&mut self) {
        let mut coords: Vec<(i32, i32)> = vec![];
        for i in 0..GRID_WIDTH as i32 {
            for j in 0..GRID_HEIGHT as i32 {
                let cell_state = self.grid[i as usize][j as usize] as i8;
                let mut live_neighbours = 0;
                for x in -1..=1 {
                    for y in -1..=1 {
                        let new_x = i + x;
                        let new_y = j + y;

                        if new_x > 0
                            && new_y > 0
                            && new_x < GRID_WIDTH as i32
                            && new_y < GRID_HEIGHT as i32
                        {
                            live_neighbours += self.grid[new_x as usize][new_y as usize] as i8;
                        }
                    }
                }

                live_neighbours -= cell_state;

                if cell_state == 1 && (live_neighbours < 2 || live_neighbours > 3) {
                    coords.push((i, j));
                } else if cell_state == 0 && live_neighbours == 3 {
                    coords.push((i, j));
                }
            }
        }
        for coord in coords.iter() {
            self.grid[coord.0 as usize][coord.1 as usize] ^= true;
        }
    }

    fn handle_keyboard(&mut self, ctx: &mut Context) {
        if ctx
            .keyboard
            .is_key_pressed(ggez::input::keyboard::KeyCode::Space)
        {
            self.pause_play();
        } else if ctx
            .keyboard
            .is_key_pressed(ggez::input::keyboard::KeyCode::C)
        {
            self.clear_grid();
            self.random_fill(2000);
        }
    }
    fn pause_play(&mut self) {
        match self.state {
            State::Running => self.state = State::Pause,
            State::Pause => self.state = State::Running,
        }
    }
    fn clear_grid(&mut self) {
        self.grid = vec![vec![false; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
    }
    fn random_fill(&mut self, num_to_fill: u32) {
        let mut rng = thread_rng();
        for _ in 0..num_to_fill {
            let random_x: usize = rng.gen_range(1..GRID_WIDTH as usize - 1);
            let random_y: usize = rng.gen_range(1..GRID_HEIGHT as usize - 1);
            self.grid[random_x][random_y] = true;
        }
    }
    fn draw_generation_text(&self, canvas: &mut Canvas) {
        let dest_point = vec2(5., 5.);
        canvas.draw(
            Text::new(format!("generation: {}", self.generation)).set_scale(30.),
            dest_point,
        );
    }
    fn draw_grid(&self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        for i in 0..GRID_WIDTH as i32 {
            for j in 0..GRID_HEIGHT as i32 {
                if self.grid[i as usize][j as usize] {
                    canvas.draw(
                        &Mesh::new_rectangle(
                            ctx,
                            DrawMode::fill(),
                            Rect::new(
                                (i * CELL_SIZE as i32) as f32,
                                (j * CELL_SIZE as i32) as f32,
                                CELL_SIZE,
                                CELL_SIZE,
                            ),
                            Color::WHITE,
                        )?,
                        DrawParam::default(),
                    );
                }
            }
        }
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.time.check_update_time(self.target_fps) {
            match self.state {
                State::Running => {
                    self.update_grid_state();
                    self.generation += 1;
                }
                State::Pause => println!("The game is paused"),
            }
            self.handle_keyboard(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.draw_generation_text(&mut canvas);
        self.draw_grid(&mut canvas, ctx)?;

        // else {
        //     canvas.draw(
        //         &Mesh::new_rectangle(
        //             ctx,
        //             DrawMode::stroke(1.),
        //             Rect::new(
        //                 (i * CELL_SIZE as i32) as f32,
        //                 (j * CELL_SIZE as i32) as f32,
        //                 CELL_SIZE,
        //                 CELL_SIZE,
        //             ),
        //             Color::WHITE,
        //         )?,
        //         DrawParam::default(),
        //     );
        // }
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Game-of-Life", "HichamLamine")
        .window_setup(WindowSetup::default().title("Game of Life"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let mut game = GameState::new(&mut ctx);

    game.random_fill(2000);
    event::run(ctx, event_loop, game)
}


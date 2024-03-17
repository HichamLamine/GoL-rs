use std::cell;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    winit::dpi::Pixel,
    Context, ContextBuilder, GameResult,
};
use rand::{thread_rng, Rng};

const CELL_SIZE: f32 = 10.;
const GRID_WIDTH: f32 = 100.;
const GRID_HEIGHT: f32 = 100.;
const WINDOW_WIDTH: f32 = CELL_SIZE * GRID_WIDTH;
const WINDOW_HEIGHT: f32 = CELL_SIZE * GRID_HEIGHT;

struct GameState {
    grid: Vec<Vec<bool>>,
    generation: usize,
    target_fps: u32,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        ctx.gfx.set_window_title("Game-of-Life");
        Self {
            grid: vec![vec![false; GRID_HEIGHT as usize]; GRID_WIDTH as usize],
            generation: 0,
            target_fps: 20,
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
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.time.check_update_time(self.target_fps) {
            self.update_grid_state();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

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
            }
        }

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

    let generated_cells: u32 = 2000;
    let mut rng = thread_rng();

    for i in 0..generated_cells {
        let random_x: usize = rng.gen_range(1..GRID_WIDTH as usize - 1);
        let random_y: usize = rng.gen_range(1..GRID_HEIGHT as usize - 1);
        game.grid[random_x][random_y] = true;
    }

    event::run(ctx, event_loop, game)
}


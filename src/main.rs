use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    winit::dpi::Pixel,
    Context, ContextBuilder, GameResult,
};

const CELL_SIZE: f32 = 20.;
const GRID_WIDTH: f32 = 50.;
const GRID_HEIGHT: f32 = 50.;
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
            target_fps: 1,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.time.check_update_time(self.target_fps) {
            println!("Hi");
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
    game.grid[0][3] = true;
    event::run(ctx, event_loop, game)
}


use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::glam::*;
use ggez::graphics::{self, Color, Canvas, Mesh, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::path;

// Define the size of the grid.
const GRID_WIDTH: usize = 120; // Alternatively 80
const GRID_HEIGHT: usize = 90; // Alternatively 60
const CELL_SIZE: f32 = 15.0; // Alternatively 10.0

/// Struct representing the game state.
struct MainState {
    grid: Vec<Vec<bool>>,
    next_grid: Vec<Vec<bool>>,
    paused: bool,
}

impl MainState {
    /// Create a new game state.
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut s = MainState {
            grid: vec![vec![false; GRID_WIDTH]; GRID_HEIGHT],
            next_grid: vec![vec![false; GRID_WIDTH]; GRID_HEIGHT],
            paused: true, // Start in paused mode to allow pattern setup
        };

        // Initialize the grid with a simple pattern (e.g., a glider)
        s.grid[1][2] = true;
        s.grid[2][3] = true;
        s.grid[3][1] = true;
        s.grid[3][2] = true;
        s.grid[3][3] = true;

        Ok(s)
    }

    /// Count the live neighbors of a cell.
    fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let xs = [x.wrapping_sub(1), x, x + 1];
        let ys = [y.wrapping_sub(1), y, y + 1];

        for &i in &ys {
            if i >= GRID_HEIGHT {
                continue;
            }
            for &j in &xs {
                if j >= GRID_WIDTH || (i == y && j == x) {
                    continue;
                }
                if self.grid[i][j] {
                    count += 1;
                }
            }
        }

        count
    }

    /// Update the grid based on Game of Life rules.
    fn update_grid(&mut self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let live_neighbors = self.live_neighbor_count(x, y);
                self.next_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    // Rule 1: Any live cell with two or three live neighbours survives.
                    (true, 2) | (true, 3) => true,
                    // Rule 2: Any dead cell with three live neighbours becomes a live cell.
                    (false, 3) => true,
                    // Rule 3: All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                    _ => false,
                };
            }
        }

        // Swap grids for next iteration
        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    /// Toggle the paused state
    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Toggle the state of a cell at a given position
    fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.grid[y][x] = !self.grid[y][x];
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.paused {
            self.update_grid();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.grid[y][x] {
                    let rect = Rect::new(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    );

                    let cell = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;
                    canvas.draw(&cell, graphics::DrawParam::default());
                }
            }
        }

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == ggez::input::mouse::MouseButton::Left {
            let grid_x = (x / CELL_SIZE) as usize;
            let grid_y = (y / CELL_SIZE) as usize;
            self.toggle_cell(grid_x, grid_y);
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        use ggez::input::keyboard::KeyCode;
        match input.keycode {
            Some(KeyCode::Space) => {
                self.toggle_pause();
            }
            Some(KeyCode::C) => {
                // Clear the grid
                self.grid = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
            }
            Some(KeyCode::Escape) => {
                // Quit the game
                _ctx.request_quit();
            }
            _ => (),
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let (grid_width, grid_height) = (
        (GRID_WIDTH as f32) * CELL_SIZE,
        (GRID_HEIGHT as f32) * CELL_SIZE,
    );
    let cb = ContextBuilder::new("krida", "nealpro")
        .window_setup(conf::WindowSetup::default().title("Krida - Game of Life"))
        .window_mode(conf::WindowMode::default().dimensions(grid_width, grid_height))
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
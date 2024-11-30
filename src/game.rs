use ggez::event::EventHandler;
use ggez::glam::*;
use ggez::graphics::{self, Canvas, Color, Mesh, Rect};
use ggez::timer;
use ggez::{Context, GameResult};
use rand::random;
use std::time::Duration;

// Define the size of the grid.
pub const GRID_WIDTH: usize = 120; // Alternatively 80
pub const GRID_HEIGHT: usize = 90; // Alternatively 60
pub const CELL_SIZE: f32 = 15.0; // Alternatively 10.0
const DEFAULT_UPDATE_DELAY_MILISECONDS: u64 = 100;
const DEFAULT_UPDATE_DELAY: Duration = Duration::from_millis(DEFAULT_UPDATE_DELAY_MILISECONDS);

/// Struct representing the game state.
pub struct MainState {
    grid: Vec<Vec<bool>>,
    next_grid: Vec<Vec<bool>>,
    paused: bool,
    update_delay: Duration,
    change_update_delay: Duration,
}

impl MainState {
    /// Create a new game state.
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut s = MainState {
            grid: vec![vec![false; GRID_WIDTH]; GRID_HEIGHT],
            next_grid: vec![vec![false; GRID_WIDTH]; GRID_HEIGHT],
            paused: true, // Start in paused mode to allow pattern setup
            update_delay: DEFAULT_UPDATE_DELAY,
            change_update_delay: DEFAULT_UPDATE_DELAY,
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
        // Check the 3x3 grid around the cell
        // The following code wraps around the edges of the grid.
        // This is a common technique in Game of Life implementations.
        // However, it is not the only way to handle the edges.
        // Infact, the more consistent way is to ignore the edges, because the Game of Life is played on an infinite grid.
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

    /// Set cells to a random state
    fn randomize(&mut self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                self.grid[y][x] = random();
            }
        }
    }

    /// Set cells to a random state, but with a much lower probability of being alive
    fn randomize_sparse(&mut self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                self.grid[y][x] = random::<f32>() < 0.1;
            }
        }
    }

    /// Decrease the update delay step
    fn decrease_update_delay_step(&mut self) {
        if self.change_update_delay > Duration::from_millis(10) {
            self.change_update_delay -= Duration::from_millis(10);
        }
    }

    /// Increase the update delay step
    fn increase_update_delay_step(&mut self) {
        if self.change_update_delay < Duration::from_millis(100) {
            self.change_update_delay += Duration::from_millis(10);
        }
    }

    /// Increase the update delay
    fn increase_update_delay(&mut self) {
        self.update_delay += self.change_update_delay;
    }

    /// Decrease the update delay if it is greater than the minimum delay
    fn decrease_update_delay(&mut self) {
        if self.update_delay > DEFAULT_UPDATE_DELAY
            && (self.update_delay - self.change_update_delay) > DEFAULT_UPDATE_DELAY
        {
            self.update_delay -= self.change_update_delay;
        }
    }

    /// Reset update delay to default
    fn reset_update_delay(&mut self) {
        self.update_delay = DEFAULT_UPDATE_DELAY;
        self.change_update_delay = DEFAULT_UPDATE_DELAY;
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.paused {
            self.update_grid();
            timer::sleep(self.update_delay);
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

                    let cell =
                        Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;
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
            Some(KeyCode::P) => {
                // Randomize the grid
                self.randomize();
            }
            Some(KeyCode::R) => {
                // Randomize the grid sparsely
                self.randomize_sparse();
            }
            Some(KeyCode::Up) => {
                // Increase the update delay
                self.increase_update_delay();
            }
            Some(KeyCode::Down) => {
                // Decrease the update delay
                self.decrease_update_delay();
            }
            Some(KeyCode::RShift) => {
                // Reset the update delay
                self.reset_update_delay();
            }
            Some(KeyCode::Right) => {
                // Increase the update delay step
                self.increase_update_delay_step();
            }
            Some(KeyCode::Left) => {
                // Decrease the update delay step
                self.decrease_update_delay_step();
            }
            _ => (),
        }
        Ok(())
    }
}

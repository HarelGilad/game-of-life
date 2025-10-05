use macroquad::prelude::*;

mod grid;
mod game;

use grid::Cell;
use grid::Grid;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 40;
const UI_HEIGHT: f32 = 30.0;

const FRAME_RATE: usize = 10;


fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        window_width: ((CELL_SIZE * GRID_WIDTH as f32) as i32),
        window_height: ((CELL_SIZE * GRID_HEIGHT as f32 + UI_HEIGHT) as i32),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    let mut paused = true;
    let mut frame = 0;

    loop {
        clear_background(BLACK);

        // Draw UI bar at top
        draw_rectangle(0.0, 0.0, screen_width(), UI_HEIGHT, DARKGRAY);
        let status = if paused { "PAUSED" } else { "RUNNING" };
        draw_text(
            &format!("{} | SPACE: Play/Pause | C: Clear | Click: Draw/Erase", status),
                  10.0,
                  20.0,
                  20.0,
                  ORANGE
        );

        // Draw cells
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.cells[y][x] == Cell::Alive {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE + UI_HEIGHT,
                        CELL_SIZE,
                        CELL_SIZE,
                        ORANGE
                    );
                }
            }
        }

        // Input handling
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::C) {
            paused = true;
            grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_y > UI_HEIGHT {
                let x = (mouse_x / CELL_SIZE) as usize;
                let y = ((mouse_y - UI_HEIGHT) / CELL_SIZE) as usize;
                grid.toggle_cell_state(x, y);
            }
        }

        // Update screen
        if !paused && frame == 0 {
            grid = game::next_generation(&grid);
        }

        frame = (frame + 1) % FRAME_RATE;
        next_frame().await;
    }
}

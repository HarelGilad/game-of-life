use crate::grid::Cell;
use crate::grid::Grid;

fn count_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip curr cell
            }

            let cell_x = x as i32 + dx;
            let cell_y = y as i32 + dy;

            if cell_x >= 0 && cell_x < grid.width as i32 && cell_y >= 0 && cell_y < grid.height as i32 {
                if grid.cells[cell_y as usize][cell_x as usize].is_alive() {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn next_generation(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new(grid.width, grid.height);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let neighbors = count_neighbors(grid, x, y);
            let cell = grid.cells[y][x];

            new_grid.cells[y][x] = match (cell, neighbors) {
                (Cell::Alive, 2..=3) => Cell::Alive,
                (Cell::Dead, 3) => Cell::Alive,
                _ => Cell::Dead
            };
        }
    }

    new_grid
}
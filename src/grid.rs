#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Dead,
    Alive
}

impl Cell {
    // Methods
    pub fn is_alive(&self) -> bool {
        matches!(self, Cell::Alive)
    }
}


pub struct Grid {
    // Field
    pub cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize
}

impl Grid {
    // Constructor
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![Cell::Dead; width]; height],
            width,
            height
        }
    }

    // Methods
    pub fn toggle_cell_state(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[y][x] = match self.cells[y][x] {
                Cell::Alive => Cell::Dead,
                Cell::Dead => Cell::Alive
            };
        }
    }
}
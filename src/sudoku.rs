const BOARD_SIZE: usize = 9;

pub struct Sudoku {
    pub grid: Vec<Vec<usize>>,
}

impl Sudoku {
    #[inline]
    pub fn index(&mut self, index: usize) -> &usize {
        self.mut_index(index)
    }

    #[inline]
    pub fn mut_index(&mut self, index: usize) -> &mut usize {
        let (x, y) = self.get_x_y(index);
        &mut self.grid[y][x]
    }

    #[inline]
    pub fn get_x_y(&self, index: usize) -> (usize, usize) {
        (index % BOARD_SIZE, index / BOARD_SIZE)
    }
}

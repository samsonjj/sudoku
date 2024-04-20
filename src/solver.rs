const BOARD_SIZE: usize = 9;

use crate::sudoku::Sudoku;

pub struct Solver1<'a> {
    sudoku: &'a mut Sudoku,
}

impl<'a> Solver1<'a> {
    pub fn solve(sudoku: &'a mut Sudoku) -> Result<(), String> {
        let mut _self = Self { sudoku };
        match _self.solve_r(0) {
            true => Ok(()),
            false => Err("No solution".to_string()),
        }
    }

    fn solve_r(&mut self, index: usize) -> bool {
        if index == 81 {
            return true;
        }
        if self.sudoku.index(index) != &0 {
            return self.solve_r(index + 1);
        }
        for val in 1..=BOARD_SIZE {
            if self.is_valid(index, val) {
                *self.sudoku.mut_index(index) = val;
                if self.solve_r(index + 1) {
                    return true;
                }
            }
        }
        *self.sudoku.mut_index(index) = 0;

        false
    }

    fn is_valid(&self, index: usize, val: usize) -> bool {
        let (x1, y1) = self.sudoku.get_x_y(index);
        // row check
        for x2 in 0..BOARD_SIZE {
            if self.sudoku.grid[y1][x2] == val {
                return false;
            }
        }
        // column check
        for y2 in 0..BOARD_SIZE {
            if self.sudoku.grid[y2][x1] == val {
                return false;
            }
        }
        // grid check
        // (a, b) is coordinate of top-left of grid, via math magic
        let a = (x1 / 3) * 3;
        let b = (y1 / 3) * 3;
        for da in 0..3 {
            for db in 0..3 {
                let x2 = a + da;
                let y2 = b + db;
                if self.sudoku.grid[y2][x2] == val {
                    return false;
                }
            }
        }
        true
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print_string = self
            .grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|x| char::from_digit(*x as u32, 10).unwrap())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{print_string}").unwrap();
        Ok(())
    }
}

pub struct Sudoku {
    pub grid: Vec<Vec<u32>>,
}

impl Sudoku {
    pub fn solve(data: Vec<Vec<u32>>) -> Result<Sudoku, &'static str> {
        let mut sudoku = Self { grid: data };
        if sudoku.solve_r(0) {
            return Ok(sudoku);
        }
        Err("No solution")
    }

    fn solve_r(&mut self, index: usize) -> bool {
        if index == 81 {
            return true;
        }
        if self.at_index(index) != 0 {
            return self.solve_r(index + 1);
        }
        for val in 1..=9 {
            if self.is_valid(index, val) {
                self.set_index(index, val);
                if self.solve_r(index + 1) {
                    return true;
                }
            }
        }
        self.set_index(index, 0);
        return false;
    }

    fn set_index(&mut self, index: usize, val: u32) {
        let (x, y) = self.get_x_y(index);
        self.grid[y][x] = val;
    }

    fn at_index(&self, index: usize) -> u32 {
        let (x, y) = self.get_x_y(index);
        return self.grid[y][x];
    }

    #[inline]
    fn get_x_y(&self, index: usize) -> (usize, usize) {
        (index % 9, index / 9)
    }

    fn is_valid(&self, index: usize, val: u32) -> bool {
        let (x1, y1) = self.get_x_y(index);
        // row check
        for x2 in 0..9 {
            if self.grid[y1][x2] == val {
                return false;
            }
        }
        // column check
        for y2 in 0..9 {
            if self.grid[y2][x1] == val {
                return false;
            }
        }
        // grid check
        let a = (x1 / 3) * 3;
        let b = (y1 / 3) * 3;
        for da in 0..3 {
            for db in 0..3 {
                let x2 = a + da;
                let y2 = b + db;
                if self.grid[y2][x2] == val {
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
                    .map(|x| char::from_digit(*x, 10).unwrap())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{print_string}").unwrap();
        Ok(())
    }
}

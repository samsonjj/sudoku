// use std::collections::HashSet;

use crate::sudoku::{Sudoku, BOARD_SIZE};

#[derive(Clone, Debug)]
pub struct MyHashSet(pub u64);

impl MyHashSet {
    pub fn contains(&self, x: &usize) -> bool {
        self.0 >> (x - 1) & 1 == 1
    }
    pub fn remove(&mut self, x: &usize) {
        self.0 &= !(0b000000001u64 << (x - 1));
    }
    pub fn insert(&mut self, x: usize) {
        self.0 |= 0b000000001u64 << (x - 1);
    }
    pub fn new() -> Self {
        Self(0)
    }
    pub fn extend(&mut self, data: [usize; 9]) {
        for x in data.iter() {
            self.insert(*x);
        }
    }
    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
    pub fn get_last(&self) -> usize {
        self.0.trailing_zeros() as usize + 1
    }
}

type HashSet = MyHashSet;

pub struct Solver2<'a> {
    sudoku: &'a mut Sudoku,
    pub possible: Vec<Vec<HashSet>>,
}

impl<'a> Solver2<'a> {
    pub fn new(sudoku: &'a mut Sudoku) -> Self {
        let mut nine_set = HashSet::new();
        nine_set.extend([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut possible = vec![vec![nine_set.clone(); 9]; 9];
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let val = sudoku.get((x, y));
                if val != 0 {
                    possible[y][x] = {
                        let mut hs = HashSet::new();
                        hs.insert(val);
                        hs
                    }
                }
            }
        }
        Self { sudoku, possible }
    }
    pub fn solve(sudoku: &'a mut Sudoku) -> Result<(), String> {
        let mut _self = Self::new(sudoku);
        _self.solve_r();
        Ok(())
    }
    pub fn apply(&mut self, index: (usize, usize)) {
        let val = self.sudoku.get(index);
        if val == 0 {
            return;
        }
        // row
        for x in 0..BOARD_SIZE {
            if x == index.0 {
                continue;
            }
            self.possible[index.1][x].remove(&val);
        }
        // column
        for y in 0..BOARD_SIZE {
            if y == index.1 {
                continue;
            }
            self.possible[y][index.0].remove(&val);
        }
        // box
        let a = (index.0 / 3) * 3;
        let b = (index.1 / 3) * 3;
        for da in 0..3 {
            for db in 0..3 {
                let x2 = a + da;
                let y2 = b + db;
                if x2 == index.0 && y2 == index.1 {
                    continue;
                }
                // println!("removing {} from {},{}", val, x2, y2);
                self.possible[y2][x2].remove(&val);
            }
        }
    }
    pub fn congrate(&mut self) -> bool {
        let mut changed = false;
        // rows
        for y in 0..BOARD_SIZE {
            for val in 1..=BOARD_SIZE {
                let mut possible_count = 0;
                let mut index = 0;
                for x in 0..BOARD_SIZE {
                    if self.possible[y][x].contains(&val) {
                        possible_count += 1;
                        index = x;
                    }
                }
                if possible_count == 1 && self.sudoku.get((index, y)) == 0 {
                    // TODO: just stop if we see the value set in sudoku
                    *self.sudoku.get_mut((index, y)) = val;
                    changed = true;
                }
            }
        }
        // columns
        for x in 0..BOARD_SIZE {
            for val in 1..=BOARD_SIZE {
                let mut possible_count = 0;
                let mut index = 0;
                for y in 0..BOARD_SIZE {
                    if self.possible[y][x].contains(&val) {
                        possible_count += 1;
                        index = y;
                    }
                }
                if possible_count == 1 && self.sudoku.get((x, index)) == 0 {
                    // TODO: just stop if we see the value set in sudoku
                    *self.sudoku.get_mut((x, index)) = val;
                    changed = true;
                }
            }
        }
        // grid
        // box
        for (x, y) in [
            (0, 0),
            (3, 0),
            (6, 0),
            (3, 0),
            (3, 3),
            (3, 6),
            (6, 0),
            (6, 3),
            (6, 6),
        ] {
            for val in 1..=BOARD_SIZE {
                let mut possible_count = 0;
                let mut index = (0, 0);
                for dx in 0..3 {
                    for dy in 0..3 {
                        if self.possible[y + dy][x + dx].contains(&val) {
                            possible_count += 1;
                            index = (x + dx, y + dy);
                        }
                    }
                }
                if possible_count == 1 && self.sudoku.get(index) == 0 {
                    println!("setting {:?} to {}", index, val);
                    // TODO: just stop if we see the value set in sudoku
                    *self.sudoku.get_mut(index) = val;
                    changed = true;
                }
            }
        }

        changed
    }
    pub fn solve_r(&mut self) {
        for i in 0..10000 {
            println!("{i}");
            let mut done = true;
            // account for every cell at first
            for y in 0..BOARD_SIZE {
                for x in 0..BOARD_SIZE {
                    if self.possible[y][x].len() == 1 && self.sudoku.get((x, y)) == 0 {
                        *self.sudoku.get_mut((x, y)) = self.possible[y][x].get_last(); // println!("1");
                        done = false;
                    }
                    self.apply((x, y));
                }
            }
            if self.congrate() {
                // println!("2");
                done = false;
            }
            if done {
                break;
            }
        }
    }
}

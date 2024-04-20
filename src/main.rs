use std::io::Read;

use crate::solver::Solver1;
use crate::sudoku::Sudoku;

mod parser;
mod solver;
mod sudoku;

fn read_stdin() -> String {
    let mut stdin = std::io::stdin();
    let mut buf = vec![];
    stdin.read_to_end(&mut buf).expect("failed to read stdin");
    String::from_utf8(buf).unwrap()
}

fn main() -> Result<(), String> {
    let input = read_stdin();
    let grid = parser::parse(input.as_str());
    let mut sudoku = Sudoku { grid };

    let start = std::time::Instant::now();
    Solver1::solve(&mut sudoku)?;

    let duration = start.elapsed();
    println!("Took {duration:?} ms");
    println!("{sudoku}");
    Ok(())
}

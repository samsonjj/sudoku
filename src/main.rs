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

#[derive(Default)]
struct Args {
    time: bool,
    list: bool,
    toss: bool, // toss away the solution, who needs it!
}

fn parse_args(_args: Vec<String>) -> Args {
    let _args: Vec<&str> = _args.iter().map(|s| s.as_str()).collect();
    let mut args = Args::default();
    if _args.contains(&"--time") {
        args.time = true;
    }
    if _args.contains(&"--list") {
        args.list = true;
    }
    if _args.contains(&"--toss") {
        args.toss = true;
    }
    args
}

fn main() -> Result<(), String> {
    let input = read_stdin();

    let args = parse_args(std::env::args().collect());
    let grids = if args.list {
        parser::parse_list(input.as_str())
    } else {
        vec![parser::parse(input.as_str())]
    };
    let mut sudokus: Vec<Sudoku> = grids.into_iter().map(|grid| Sudoku { grid }).collect();

    if args.time {
        for sudoku in sudokus.iter_mut() {
            let start = std::time::Instant::now();
            Solver1::solve(sudoku)?;
            let duration = start.elapsed();
            println!("Took {duration:?}");
        }
    } else {
        for sudoku in sudokus.iter_mut() {
            Solver1::solve(sudoku)?;
        }
    }

    if !args.toss {
        for sudoku in sudokus.iter() {
            println!("{sudoku}");
        }
    }
    Ok(())
}

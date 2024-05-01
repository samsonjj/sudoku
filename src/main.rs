use std::io::Read;

use crate::solver::Solver1;
use crate::solver2::Solver2;
use crate::sudoku::Sudoku;

mod parser;
mod solver;
mod solver2;
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
    let mut set = solver2::MyHashSet::new();
    set.extend([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    set.remove(&1);
    set.remove(&4);
    println!("{:#b}", set.0);
    println!("hi");
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
            Solver2::solve(sudoku)?;
            let duration = start.elapsed();
            println!("Took {duration:?}");
            Solver1::solve(sudoku)?;
            let duration = start.elapsed();
            println!("Took {duration:?}");
        }
    } else {
        for sudoku in sudokus.iter_mut() {
            // Solver2::solve(sudoku)?;
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

// fn main() -> Result<(), String> {
//     let input = read_stdin();

//     let args = parse_args(std::env::args().collect());
//     let grids = if args.list {
//         parser::parse_list(input.as_str())
//     } else {
//         vec![parser::parse(input.as_str())]
//     };
//     let mut sudokus: Vec<Sudoku> = grids.into_iter().map(|grid| Sudoku { grid }).collect();

//     let sudoku = sudokus.get_mut(0).unwrap();
//     Solver2::solve(sudoku).unwrap();
//     println!("{sudoku}");

//     // let mut solver = Solver2::new(sudoku);
//     // solver.apply((0, 0));
//     // solver.apply((3, 0));

//     // for x in 0..sudoku::BOARD_SIZE {
//     //     println!("{:?}", solver.possible[0][x]);
//     // }

//     Ok(())
// }

use std::io::Read;

mod parser;
mod solver;

fn read_stdin() -> String {
    let mut stdin = std::io::stdin();
    let mut buf = vec![0u8; 89];
    stdin.read_exact(&mut buf).expect("failed to read stdin");
    String::from_utf8(buf).unwrap()
}

fn main() -> Result<(), &'static str> {
    let input = read_stdin();
    let grid = parser::parse(input.as_str());
    let solution = solver::Sudoku::solve(grid)?;
    solution.print();
    Ok(())
}
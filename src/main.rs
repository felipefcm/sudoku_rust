mod loader;
mod solver;
mod sudoku;

use solver::Solver;
use sudoku::Sudoku;

fn main() {
    let puzzles = loader::load_file("puzzles.csv");

    for (i, puzzle) in (&puzzles[..10000]).iter().enumerate() {
        eprintln!("Solving Sudoku #{i}");

        let mut state = Sudoku::from_string(&puzzle.0);

        let solved = Solver::solve(&mut state);
        assert!(solved);

        if solved {
            let solution = Sudoku::stringify_state(&state);
            assert_eq!(puzzle.1, solution);
        }
    }
}

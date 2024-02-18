use sudoku_rs::loader;
use sudoku_rs::solver::Solver;
use sudoku_rs::sudoku::Sudoku;

fn main() {
    let puzzles = loader::load_file("puzzles.csv");

    for (i, puzzle) in (&puzzles[..10000]).iter().enumerate() {
        println!("Solving Sudoku #{i}");

        let mut state = Sudoku::parse_state(&puzzle.0);

        let solved = Solver::solve(&mut state);
        assert!(solved);

        if solved {
            let solution = Sudoku::stringify_state(&state);
            assert_eq!(puzzle.1, solution);
        }
    }
}

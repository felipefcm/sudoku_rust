use sudoku_rs::solver::Solver;
use sudoku_rs::sudoku::Sudoku;

fn main() {
    let mut state = Sudoku::parse_state(
        "004300209005009001070060043006002087190007400050083000600000105003508690042910300",
    );

    let solved = Solver::solve(&mut state);
    if solved {
        let solution = Sudoku::stringify_state(&state);
        println!("Solved: {}", solution);
    } else {
        println!("No solution found");
    }
}

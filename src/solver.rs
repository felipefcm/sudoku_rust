use crate::sudoku::grid::Grid;
use crate::sudoku::Sudoku;

pub struct Solver {}

impl Solver {
    pub fn solve(sudoku: &mut Sudoku) -> bool {
        let solver = Solver {};
        solver.search(0, sudoku)
    }

    fn search(&self, index: usize, state: &mut Sudoku) -> bool {
        if state.is_index_fixed(index) {
            return self.bypass_fixed(index, state);
        }

        return self.make_attempts(index, state);
    }

    fn bypass_fixed(&self, index: usize, state: &mut Sudoku) -> bool {
        if Grid::is_last_cell(index) {
            return true;
        }

        return self.search(index + 1, state);
    }

    fn make_attempts(&self, index: usize, state: &mut Sudoku) -> bool {
        let n = state.grid.get_at_index(index);

        for i in n + 1..=9 {
            state.grid.set_at_index(index, i);

            if !state.is_valid(index) {
                continue;
            }

            if Grid::is_last_cell(index) {
                return true;
            }

            if self.search(index + 1, state) {
                return true;
            };
        }

        state.grid.set_at_index(index, 0);
        return false;
    }
}

use crate::sudoku::grid::Grid;
use crate::sudoku::Sudoku;

struct Node {
    index: usize,
}

pub struct Solver {}

impl Solver {
    pub fn solve(sudoku: &mut Sudoku) -> bool {
        let solver = Solver {};
        let root_node = Node { index: 0 };

        return solver.search(root_node, sudoku);
    }

    fn search(&self, node: Node, state: &mut Sudoku) -> bool {
        if state.is_index_fixed(node.index) {
            return self.bypass_fixed(node, state);
        }

        return self.make_attempts(node, state);
    }

    fn bypass_fixed(&self, node: Node, state: &mut Sudoku) -> bool {
        if Grid::is_last_cell(node.index) {
            return true;
        }

        let next_node = Node {
            index: node.index + 1,
        };

        return self.search(next_node, state);
    }

    fn make_attempts(&self, node: Node, state: &mut Sudoku) -> bool {
        let n = state.grid.get_at_index(node.index);

        for i in n + 1..=9 {
            state.grid.set_at_index(node.index, i);

            if !state.is_valid(node.index) {
                continue;
            }

            if Grid::is_last_cell(node.index) {
                return true;
            }

            let next_node = Node {
                index: node.index + 1,
            };

            if self.search(next_node, state) {
                return true;
            };
        }

        state.grid.set_at_index(node.index, 0);
        return false;
    }
}

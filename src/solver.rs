use crate::sudoku::Sudoku;

struct Node {
    index: usize,
}

pub struct Solver {}

impl Solver {
    pub fn solve(sudoku: &mut Sudoku) -> bool {
        let solver = Solver {};

        let root_node = Node {
            index: 0, // find first empty position
        };

        solver.search(root_node, sudoku)
    }

    fn search(&self, node: Node, state: &mut Sudoku) -> bool {
        let n = state.get_at_index(node.index);

        if state.is_index_fixed(node.index) {
            if Sudoku::is_last_cell(node.index) {
                return true;
            }

            let next_node = Node {
                index: node.index + 1,
            };

            if self.search(next_node, state) {
                return true;
            };

            return false;
        }

        for i in n + 1..=9 {
            state.set_at_index(node.index, i);

            if !state.is_valid(node.index) {
                continue;
            }

            if Sudoku::is_last_cell(node.index) {
                return true;
            }

            let next_node = Node {
                index: node.index + 1,
            };

            if self.search(next_node, state) {
                return true;
            };
        }

        if !state.is_index_fixed(node.index) {
            state.set_at_index(node.index, 0);
        }

        return false;
    }
}

pub mod grid;
use grid::Grid;

const N: usize = 3;
const N2: usize = N * N;
const CELLS: usize = N2 * N2;
const EMPTY: u32 = 0;

pub struct Sudoku {
    pub grid: Grid,
    pub fixed_indices: Vec<usize>,
}

impl Sudoku {
    pub fn empty_state() -> Self {
        Self {
            grid: Grid::new(),
            fixed_indices: Vec::new(),
        }
    }

    pub fn from_string(initial_state: &str) -> Self {
        let mut sudoku = Self::empty_state();

        for (i, c) in initial_state.chars().enumerate() {
            let n = c.to_digit(10).expect("Failed to parse");
            sudoku.grid.set_at_index(i, n);

            if n != 0 {
                sudoku.fixed_indices.push(i);
            }
        }

        return sudoku;
    }

    pub fn stringify_state(&self) -> String {
        let mut state_str = String::with_capacity(CELLS);

        for i in 0..9 {
            for j in 0..9 {
                state_str.push_str(&self.grid.get_at_pos((i, j)).to_string());
            }
        }

        return state_str;
    }

    pub fn is_valid(&self, index: usize) -> bool {
        let (row, col) = Grid::index_to_position(index);
        let n = self.grid.get_at_index(index);

        if !self.is_row_valid(row, n) {
            return false;
        }

        if !self.is_col_valid(col, n) {
            return false;
        }

        if !self.is_block_valid(row, col, n) {
            return false;
        }

        return true;
    }

    fn is_row_valid(&self, row: usize, n: u32) -> bool {
        let in_row = self.grid.get_row(row).iter().filter(|&x| *x == n).count();
        in_row == 1
    }

    fn is_col_valid(&self, col: usize, n: u32) -> bool {
        let col = self.grid.get_col(col);
        let in_col = col.iter().filter(|&x| *x == n).count();

        in_col == 1
    }

    fn is_block_valid(&self, row: usize, col: usize, n: u32) -> bool {
        let b_row_index = row / N * N;
        let b_col_index = col / N * N;

        let block = self.grid.get_block(b_row_index, b_col_index);
        let in_block = block.iter().filter(|&x| *x == n).count();

        in_block == 1
    }

    pub fn is_index_fixed(&self, index: usize) -> bool {
        self.fixed_indices.contains(&index)
    }
}

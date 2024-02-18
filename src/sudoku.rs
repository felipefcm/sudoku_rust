const N: u32 = 3;
const N2: u32 = N * N;
const CELLS: u32 = N2 * N2;
const EMPTY: u32 = 0;

pub struct Sudoku {
    grid: [[u32; N2 as usize]; N2 as usize],
    fixed_indices: Vec<usize>,
}

impl Sudoku {
    pub fn empty_state() -> Self {
        Self {
            grid: [[EMPTY; N2 as usize]; N2 as usize],
            fixed_indices: Vec::new(),
        }
    }

    pub fn parse_state(initial_state: &str) -> Self {
        let mut s = Self::empty_state();

        for (i, c) in initial_state.chars().enumerate() {
            let n = c.to_digit(10).expect("Failed to parse");
            s.set_at_index(i, n);

            if n != 0 {
                s.fixed_indices.push(i);
            }
        }

        return s;
    }

    pub fn stringify_state(&self) -> String {
        let mut s = String::with_capacity(CELLS as usize);

        for i in 0..9 {
            for j in 0..9 {
                s.push_str(&self.grid[i][j].to_string());
            }
        }

        return s;
    }

    fn index_to_position(index: usize) -> (usize, usize) {
        let row = index / N2 as usize;
        let col = index % N2 as usize;

        (row, col)
    }

    pub fn get_at_pos(&self, (row, col): (usize, usize)) -> u32 {
        self.grid[row][col]
    }

    pub fn get_at_index(&self, index: usize) -> u32 {
        let pos = Self::index_to_position(index);
        self.get_at_pos(pos)
    }

    pub fn set_at_pos(&mut self, (row, col): (usize, usize), n: u32) {
        self.grid[row][col] = n;
    }

    pub fn set_at_index(&mut self, index: usize, n: u32) {
        let pos = Self::index_to_position(index);
        self.set_at_pos(pos, n);
    }

    pub fn is_valid(&self, index: usize) -> bool {
        let (row, col) = Self::index_to_position(index);
        let n = self.get_at_index(index);

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
        let in_row = self.grid[row].iter().filter(|&&x| x == n).count();
        in_row == 1
    }

    fn is_col_valid(&self, col: usize, n: u32) -> bool {
        let mut in_col = 0;
        for i in 0..9 {
            if self.grid[i][col] == n {
                in_col += 1;
            }
        }

        in_col == 1
    }

    fn is_block_valid(&self, row: usize, col: usize, n: u32) -> bool {
        let b_row_index = row / N as usize * N as usize;
        let b_col_index = col / N as usize * N as usize;

        let mut block = [0u32; N2 as usize];

        block[0] = self.grid[b_row_index][b_col_index];
        block[1] = self.grid[b_row_index][b_col_index + 1];
        block[2] = self.grid[b_row_index][b_col_index + 2];

        block[3] = self.grid[b_row_index + 1][b_col_index];
        block[4] = self.grid[b_row_index + 1][b_col_index + 1];
        block[5] = self.grid[b_row_index + 1][b_col_index + 2];

        block[6] = self.grid[b_row_index + 2][b_col_index];
        block[7] = self.grid[b_row_index + 2][b_col_index + 1];
        block[8] = self.grid[b_row_index + 2][b_col_index + 2];

        let in_block = block.iter().filter(|&&x| x == n).count();

        in_block == 1
    }

    pub fn is_last_cell(index: usize) -> bool {
        index == (CELLS - 1) as usize
    }

    pub fn is_index_fixed(&self, index: usize) -> bool {
        self.fixed_indices.contains(&index)
    }
}

use super::*;

pub struct Grid {
    grid: [[u32; N2]; N2],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: [[EMPTY; N2]; N2],
        }
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

    pub fn index_to_position(index: usize) -> (usize, usize) {
        let row = index / N2;
        let col = index % N2;

        (row, col)
    }

    pub fn get_row(&self, index: usize) -> &[u32] {
        &self.grid[index]
    }

    pub fn get_col(&self, index: usize) -> [u32; N2] {
        let mut col = [0u32; N2];

        for (i, row) in self.grid.iter().enumerate() {
            col[i] = row[index];
        }

        return col; //it will error here!
    }

    pub fn is_last_cell(index: usize) -> bool {
        index == CELLS - 1
    }

    pub fn get_block(&self, b_row_index: usize, b_col_index: usize) -> [u32; N2] {
        let mut block = [0u32; N2];

        block[0] = self.get_at_pos((b_row_index, b_col_index));
        block[1] = self.get_at_pos((b_row_index, b_col_index + 1));
        block[2] = self.get_at_pos((b_row_index, b_col_index + 2));

        block[3] = self.get_at_pos((b_row_index + 1, b_col_index));
        block[4] = self.get_at_pos((b_row_index + 1, b_col_index + 1));
        block[5] = self.get_at_pos((b_row_index + 1, b_col_index + 2));

        block[6] = self.get_at_pos((b_row_index + 2, b_col_index));
        block[7] = self.get_at_pos((b_row_index + 2, b_col_index + 1));
        block[8] = self.get_at_pos((b_row_index + 2, b_col_index + 2));

        return block;
    }
}

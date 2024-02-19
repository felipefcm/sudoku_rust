use super::*;

pub struct Grid {
    grid: [[u32; N2 as usize]; N2 as usize],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: [[EMPTY; N2 as usize]; N2 as usize],
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
        let row = index / N2 as usize;
        let col = index % N2 as usize;

        (row, col)
    }

    pub fn get_row(&self, index: usize) -> &[u32] {
        &self.grid[index]
    }

    pub fn get_col(&self, index: usize) -> [u32; N2 as usize] {
        let mut col = [0u32; N2 as usize];

        for (i, row) in self.grid.iter().enumerate() {
            col[i] = row[index];
        }

        return col; //it will error here!
    }

    pub fn is_last_cell(index: usize) -> bool {
        index == (CELLS - 1) as usize
    }
}

const N: u32 = 3;
const N2: u32 = N * N;
const CELLS: u32 = N2 * N2;
const EMPTY: u32 = 0;

type State = [[u32; N2 as usize]; N2 as usize];

struct Node {
    index: u32,
}

fn empty_state() -> State {
    [[EMPTY; N2 as usize]; N2 as usize]
}

fn main() {
    let mut state: State = empty_state();

    let root_node = Node {
        index: 0, // find first empty position
    };

    let solved = search(root_node, &mut state);
    if solved {
        let solution = stringify_state(&state);
        println!("Solved: {}", solution);
    } else {
        println!("No solution found");
    }
}

fn search(node: Node, state: &mut State) -> bool {
    let (row, col) = index_to_position(node.index);
    let n = state[row][col];

    for i in n + 1..=9 {
        state[row][col] = i;

        if !is_valid(state, (row, col)) {
            continue;
        }

        if is_last_cell(node.index) {
            return true;
        }

        let next_node = Node {
            index: node.index + 1,
        };

        if search(next_node, state) {
            return true;
        };
    }

    state[row][col] = 0;
    return false;
}

fn is_last_cell(index: u32) -> bool {
    index == CELLS - 1
}

fn is_valid(state: &State, (row, col): (usize, usize)) -> bool {
    let n = state[row][col];

    if !is_row_valid(state, row, n) {
        return false;
    }

    if !is_col_valid(state, col, n) {
        return false;
    }

    if !is_block_valid(state, row, col, n) {
        return false;
    }

    return true;
}

fn is_row_valid(state: &State, row: usize, n: u32) -> bool {
    let in_row = state[row].iter().filter(|&&x| x == n).count();
    in_row == 1
}

fn is_col_valid(state: &State, col: usize, n: u32) -> bool {
    let mut in_col = 0;
    for i in 0..9 {
        if state[i][col] == n {
            in_col += 1;
        }
    }

    in_col == 1
}

fn is_block_valid(state: &State, row: usize, col: usize, n: u32) -> bool {
    let b_row_index = row / N as usize * N as usize;
    let b_col_index = col / N as usize * N as usize;

    let mut block = [0u32; N2 as usize];

    block[0] = state[b_row_index][b_col_index];
    block[1] = state[b_row_index][b_col_index + 1];
    block[2] = state[b_row_index][b_col_index + 2];

    block[3] = state[b_row_index + 1][b_col_index];
    block[4] = state[b_row_index + 1][b_col_index + 1];
    block[5] = state[b_row_index + 1][b_col_index + 2];

    block[6] = state[b_row_index + 2][b_col_index];
    block[7] = state[b_row_index + 2][b_col_index + 1];
    block[8] = state[b_row_index + 2][b_col_index + 2];

    let in_block = block.iter().filter(|&&x| x == n).count();

    in_block == 1
}

fn index_to_position(index: u32) -> (usize, usize) {
    let row = (index / N2) as usize;
    let col = (index % N2) as usize;

    (row, col)
}

fn stringify_state(state: &State) -> String {
    let mut s = String::with_capacity(CELLS as usize);

    for i in 0..9 {
        for j in 0..9 {
            s.push_str(&state[i][j].to_string());
        }
    }

    return s;
}

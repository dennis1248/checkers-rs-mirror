// Cells used as playfield
const CELL_VALID: &str = "⬜";
const CELL_INVALID: &str = "⬛";

// Initialize the playfield, because hardcoding is stupid
pub fn init() {
    const ROWS: usize = 8;
    const COLS: usize = 8;

    let mut board = [["E"; COLS]; ROWS];

    {
        let mut switch_row: bool = false;   
        for y in 0..ROWS {
        
            let mut switch_col: bool = false;
            for x in 0..COLS {
                if switch_col {
                    if switch_row {
                        board[y][x] = CELL_VALID;
                    } else {
                        board[y][x] = CELL_INVALID;
                    }
                } else {
                    if switch_row {
                        board[y][x] = CELL_INVALID;
                    } else {
                        board[y][x] = CELL_VALID;
                    }
                }

                switch_col = flip(switch_col);
            }
        
            switch_row = !switch_row;
        }
    } 
}

// Flip booleans
fn flip(var: bool) -> bool {
    if var {
        false
    } else {
        true
    }
}

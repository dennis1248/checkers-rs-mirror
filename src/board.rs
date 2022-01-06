// Cells used as playfield
const CELL_VALID: &str = "⬜";
const CELL_INVALID: &str = "⬛";

const ROWS: usize = 8;
const COLS: usize = 8;

// Initialize the playfield, because hardcoding is stupid
pub fn init() -> [[&'static str; COLS]; ROWS] {
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

                switch_col = !switch_col;
            }
        
            switch_row = !switch_row;
        }
    } 

    board
}

pub fn generate(board_layout: &[[&str; COLS]; ROWS]) {
    println!("{:?}", board_layout);
}

pub fn print() {
    
}

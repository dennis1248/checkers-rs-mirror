// Cells used as playfield
const CELL_VALID: &str = "⬜";
const CELL_INVALID: &str = "⬛";

// Checkerboard size, default is 8x8
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

// Generate and return current state of board for display
pub fn render(board_layout: &[[&str; COLS]; ROWS]) -> String {
    let mut gen: String = "".to_string();

    for y in 0..ROWS {
        for x in 0..COLS {
            gen = format!("{}{}", gen, board_layout[y][x]);
        }
        gen = format!("{}{}", gen, "\n")
    }

    gen
}

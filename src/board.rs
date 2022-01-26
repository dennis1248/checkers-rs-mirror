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

// Combine board and stones location, return new board with stones placed
pub fn combine<'a>(board_layout: &'a [[&str; COLS]; ROWS], stones_location: &'a [[&str; COLS]; ROWS]) -> [[&'a str; COLS]; ROWS] {
    let mut combined: [[&str; COLS]; ROWS] = *board_layout;

    // Define player pieces
    let player_one = "🔴";
    let player_two = "🔵";

    for x in 0..COLS {
        for y in 0..ROWS {
            let stones_selector = stones_location[x][y].to_string();

            if stones_selector == "X" {
                combined[x][y] = player_one;
            } else if stones_selector == "Y" {
                combined[x][y] = player_two;
            }
        }
    }

    combined
}

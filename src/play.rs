// Checkerboard size, default is 8x8
const ROWS: usize = 8;
const COLS: usize = 8;

// Generate and return stone location array 
pub fn setup() -> [[&'static str; COLS]; ROWS]  {
    let mut stones_location = [["E"; COLS]; ROWS];
    let player_two_first_row = COLS - 2;

    // Define player characters
    let player_one_char: &str = "X";
    let player_two_char: &str = "Y";

    // Check if even or uneven
    // If even cell is non-playable cell
    for x in 0..2 {
        if let 1 = x % 2 {
            for y in 0..COLS {
                if let 1 = y % 2 {
                    stones_location[x][y] = player_one_char; 
                }
            }
        } else {
            for y in 0..COLS {
                if let 0 = y % 2 {
                    stones_location[x][y] = player_one_char; 
                }
            }
        }
    }
    
    for x in player_two_first_row..ROWS {
        if let 1 = x % 2 {
            for y in 0..COLS {
                if let 1 = y % 2 {
                    stones_location[x][y] = player_two_char; 
                }
            }
        } else {
            for y in 0..COLS {
                if let 0 = y % 2 {
                    stones_location[x][y] = player_two_char; 
                }
            }
        }
    }

    stones_location
}

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

// Count all stones currently on the board for both players and return count
pub fn count_player_stones(stones_location: &[[&str; COLS]; ROWS]) -> [usize; 2] {
    let mut player_one_counter: usize = 0;
    let mut player_two_counter: usize = 0;

    for x in 0..COLS {
        for y in 0..ROWS {
            if stones_location[x][y] == "X" {
                player_one_counter = player_one_counter + 1;
            } else if stones_location[x][y] == "Y" {
                player_two_counter = player_two_counter + 1;
            }
        }
    }
   
    [player_one_counter, player_two_counter]
}

// Check if player won
// Purely for testing now
pub fn check_for_win(&player_stones_log: &[usize; 2]) -> [bool ;2] {
    let win: bool;
    let player: bool;

    if &player_stones_log[0] <= &0 {
        win = true;
        player = false;
    } else if &player_stones_log[1] <= &0 {
        win = true;
        player = true;
    } else {
        win = false;
        player = false;
    }

    [win, player]
}

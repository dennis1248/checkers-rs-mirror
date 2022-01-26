mod board; // Manages rendering and generating of the board
mod play; // Manages player stones and moves

fn main() {
    // Generate gameboard and player stones location
    let board_layout = board::init();
    let stones_location = play::setup();

    // Main game loop
    loop {
        let active_board_current = board::combine(&board_layout, &stones_location);
        let board_render = board::render(&active_board_current);
        println!("{}", board_render);

        if play::check_for_win() {
            break;
        }
    }
}



mod board; // Manages rendering and generating of the board
mod play; // Manages player stones, moves and logs

fn main() {
    // Initial setup
    // Generate gameboard and player stones location
    let board_layout = board::init();
    let stones_location = play::setup();

    let mut player_stones_log: [usize; 2];

    // The player which may make a turn this round
    // true = Player one
    // false = Player two
    let mut player_in_play: bool = true;

    // Main game loop
    loop {
        let active_board_current = board::combine(&board_layout, &stones_location);
        let board_render = board::render(&active_board_current);
        println!("{}", board_render);
        
        // Player turns to be added here
        play::player_move(&player_in_play); 

        // Collect and provide logs/game info
        player_stones_log = play::count_player_stones(&stones_location);

       println!("Gamelog\n-------------\nPlayer one: {}\nPlayer two: {}", 
                player_stones_log[0], 
                player_stones_log[1]); 

        // Check for game win, stop game on win state reached
        let win_state = play::check_for_win(&player_stones_log);

        // End game if win state is reached, otherwise give turn to next player
        if win_state[0] {
            println!("{} has won the game!", win_state[1]);
            break; 
        } else {
            player_in_play = !player_in_play;
        }
    }
}

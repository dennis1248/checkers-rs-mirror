mod board;
mod play;


fn main() {
    let board_layout = board::init();
    let board_rendered = board::render(&board_layout);
    println!("{}", board_rendered);

    let gameboard = play::setup();

    let gameboard_rendered = board::render(&gameboard);

    println!("{}", gameboard_rendered);
}



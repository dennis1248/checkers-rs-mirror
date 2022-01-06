mod board;

fn main() {
    let board_layout = board::init();

    let board_current_render = board::render(&board_layout);
    println!("{}", board_current_render)
}

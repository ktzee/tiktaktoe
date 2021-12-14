use std::io;
// TODO: Maybe it should be an Option<XorO>
enum XorO {
    X,
    O,
    Empty,
}

impl Default for XorO {
    fn default() -> Self {
        XorO::Empty
    }
}
impl std::fmt::Display for XorO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XorO::X => write!(f, "X"),
            XorO::O => write!(f, "O"),
            XorO::Empty => write!(f, "."),
        }
    }
}

#[derive(Default)]
struct GameState {
    board: [XorO; 9],
}

impl GameState {
    //stub
}

fn show_board(game: &GameState) {
    println!(" _______ ");
    println!(
        "| {} {} {} |",
        &game.board[0], &game.board[1], &game.board[2]
    );
    println!(
        "| {} {} {} |",
        &game.board[3], &game.board[4], &game.board[5]
    );
    println!(
        "| {} {} {} |",
        &game.board[6], &game.board[7], &game.board[8]
    );
    println!(" ------- ");
}

fn player_input() {
    println!("What do you want to play?");
    println!("X (press x), O (press o)");
    // TODO: use if let to only accept X or O?
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice.");
    println!("You chose {}", choice);
    println!("Which cell do you want to play in? (Press 1-9)");
    // TODO: use if let to only accept 1 to 9?
    let mut pos = String::new();
    io::stdin()
        .read_line(&mut pos)
        .expect("Failed to read choice.");
    println!("You played at position {}", pos);
}

fn main() {
    let game = GameState::default();

    loop {
        show_board(&game);
        player_input();
    }
}

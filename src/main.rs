use std::io;
// TODO: Maybe it should be an Option<XorO>
enum XorO {
    X,
    O,
    Empty,
}

struct GameState {
    board: [XorO; 9],
}

impl GameState {
    pub fn new() -> GameState {
        // TODO: Very ugly. There must be a better way to have default values
        GameState {
            board: [
                XorO::Empty,
                XorO::Empty,
                XorO::Empty,
                XorO::Empty,
                XorO::Empty,
                XorO::Empty,
                XorO::Empty,
                XorO::Empty,
                XorO::Empty,
            ],
        }
    }
}

fn print_cell(c: &XorO) -> String {
    let text = match c {
        XorO::X => String::from("X"),
        XorO::O => String::from("O"),
        XorO::Empty => String::from("."),
    };
    text
}
fn show_board(game: &GameState) {
    println!(" _______ ");
    println!(
        "| {} {} {} |",
        print_cell(&game.board[0]),
        print_cell(&game.board[1]),
        print_cell(&game.board[2])
    );
    println!(
        "| {} {} {} |",
        print_cell(&game.board[3]),
        print_cell(&game.board[4]),
        print_cell(&game.board[5])
    );
    println!(
        "| {} {} {} |",
        print_cell(&game.board[6]),
        print_cell(&game.board[7]),
        print_cell(&game.board[8])
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
    let game = GameState::new();

    loop {
        show_board(&game);
        player_input();
    }
}

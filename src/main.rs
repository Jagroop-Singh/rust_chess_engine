use std::io::{stdin, stdout, Write};
pub mod board;

#[derive(Debug, Copy, Clone)]
pub enum COLOR {
    BLACK,
    WHITE,
}

#[derive(Debug, Copy, Clone)]
pub enum Pieces {
    Pawn(COLOR),
    Rook(COLOR),
    Knight(COLOR),
    Bishop(COLOR),
    Queen(COLOR),
    King(COLOR),
}

impl Pieces {
    fn move_piece(&self) -> (bool, COLOR) {
        use Pieces::*;

        match *self {
            Pawn(c) => (true, c),
            Rook(c) => (false, c),
            Knight(c) => (true, c),
            Bishop(c) => (false, c),
            Queen(c) => (true, c),
            King(c) => (false, c),
        }
    }
}

fn main() {
    // Create Board
    let board_init: [[Option<Pieces>; 8]; 8] = [[None; 8]; 8];
    let mut board = board::create_board(board_init);

    // Print Board
    for i in board {
        println!("{:?}", i);
    }
    println!("Do you want to play a game?");
    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Faulty String Input");
    match response.as_str() {
        "y\n" => (),
        "yes\n" => (),
        "Y\n" => (),
        "Yes\n" => (),
        "YES\n" => (),
        other => {
            println!("{:?}", other);
            return;
        }
    }

    loop {
        println!("Which Piece Do you Want to Move by R-C");
        stdin().read_line(&mut response).expect("Faulty String");
        let mut x: u8;
        let mut y: u8;
        board[2][1] = board[1][1];
        board[1][1] = None;
        board::print_board(board.clone());
    }

    // Print if piece can move
    // for i in board {
    //     for j in i {
    //         match j {
    //             Some(piece) => {
    //                 println!("{:?}", piece.move_piece());
    //             }
    //             None => {
    //                 println!("no");
    //             }
    //         }
    //     }
    // }
    // Print Map
}

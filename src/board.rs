use super::Pieces;
use super::COLOR;
pub fn create_board(mut board: [[Option<Pieces>; 8]; 8]) -> [[Option<Pieces>; 8]; 8] {
    // Add elements to Board
    for i in 0..8 {
        board[1][i] = Some(Pieces::Pawn(COLOR::WHITE));
        board[6][i] = Some(Pieces::Pawn(COLOR::BLACK));
    }
    board[0][0] = Some(Pieces::Rook(COLOR::WHITE));
    board[0][1] = Some(Pieces::Knight(COLOR::WHITE));
    board[0][2] = Some(Pieces::Bishop(COLOR::WHITE));
    board[0][3] = Some(Pieces::Queen(COLOR::WHITE));
    board[0][4] = Some(Pieces::King(COLOR::WHITE));
    board[0][5] = Some(Pieces::Bishop(COLOR::WHITE));
    board[0][6] = Some(Pieces::Knight(COLOR::WHITE));
    board[0][7] = Some(Pieces::Rook(COLOR::WHITE));
    board[7][0] = Some(Pieces::Rook(COLOR::BLACK));
    board[7][1] = Some(Pieces::Knight(COLOR::BLACK));
    board[7][2] = Some(Pieces::Bishop(COLOR::BLACK));
    board[7][3] = Some(Pieces::Queen(COLOR::BLACK));
    board[7][4] = Some(Pieces::King(COLOR::BLACK));
    board[7][5] = Some(Pieces::Bishop(COLOR::BLACK));
    board[7][6] = Some(Pieces::Knight(COLOR::BLACK));
    board[7][7] = Some(Pieces::Rook(COLOR::BLACK));
    return board;
}

pub fn print_board(board: [[Option<Pieces>; 8]; 8]) {
    for i in board {
        println!("{:?}", i);
    }
}

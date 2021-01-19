
use crate::board::*;

const MAX_DEPTH: u32 = 4;

pub struct Move {
    pub start: (usize, usize),
    pub end: (usize, usize),
    value: i32,
}
impl PartialEq for Move {
    fn eq(&self, mov: &Move) -> bool {
        self.value == mov.value
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Move) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Eq for Move {}
impl Ord for Move {
    fn cmp(&self, other: &Move) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl Move {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize, points: i32) -> Self {
        Move {
            start: (x0, y0),
            end: (x1, y1),
            value: points,
        }
    }
    pub fn max() -> Self {
        Move {
            start: (0, 0),
            end: (0, 0),
            value: i32::MAX,
        }
    }
    pub fn min() -> Self {
        Move {
            start: (0, 0),
            end: (0, 0),
            value: i32::MIN,
        }
    }
}

pub fn points(piece: PieceType) -> i32 {
    match piece {
        PieceType::Pawn => 1,
        PieceType::Knight => 3,
        PieceType::Bishop => 3,
        PieceType::Rook => 5,
        PieceType::Queen => 9,
        PieceType::King => 900,
    }
}

pub fn maximize(board: ChessBoard, depth: u32) -> Move {
    let mut best_move = Move::min();
    /* super::log(&format!("{:?}",board.pieces));
    super::log(&board.to_string()); */

    for piece in &board.pieces {
        if let ChessPiece {
            position: Some((x, y)),
            piece_color: PieceColor::Black,
            ..
        } = piece
        {
            for (movx, movy) in board.get_possible_moves(*x, *y) {
                let value: i32 = if let Some(p) = board.get(movx, movy) {
                    points(p.piece_type)
                } else {
                    0
                };
                if depth == MAX_DEPTH {
                    best_move = std::cmp::max(best_move, Move::new(*x, *y, movx, movy, value))
                } else {
                    let mut cloned = board.clone();
                    cloned.make_move((*x, *y), (movx, movy));
                    let seq = minimize(cloned, depth + 1);
                    best_move =
                        std::cmp::max(best_move, Move::new(*x, *y, movx, movy, seq.value + value))
                }
            }
        }
    }
    best_move
}
pub fn minimize(board: ChessBoard, depth: u32) -> Move {
    let mut best_move = Move::max();
    /* super::log(&format!("{:?}",board.pieces));
    super::log(&board.to_string()); */
    for piece in &board.pieces {
        if let ChessPiece {
            position: Some((x, y)),
            piece_color: PieceColor::White,
            ..
        } = piece
        {
            for (movx, movy) in board.get_possible_moves(*x, *y) {
                let value: i32 = if let Some(p) = board.get(movx, movy) {
                    -points(p.piece_type)
                } else {
                    0
                };
                if depth == MAX_DEPTH {
                    best_move = std::cmp::min(best_move, Move::new(*x, *y, movx, movy, value))
                } else {
                    let mut cloned = board.clone();
                    cloned.make_move((*x, *y), (movx, movy));
                    let seq = maximize(cloned, depth + 1);
                    best_move =
                        std::cmp::min(best_move, Move::new(*x, *y, movx, movy, seq.value + value))
                }
            }
        }
    }
    best_move
}

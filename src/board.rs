//==============================================================//
//  STRUCTS AND ENUMS
//==============================================================//
use super::log;
/// Grid representation of the chess board
#[derive(Clone)]
pub struct ChessBoard {
    pub board: [[Option<usize>; 8]; 8],
    pub pieces: [ChessPiece; 32],
}
#[derive(Clone, Debug)]
pub struct ChessPiece {
    pub piece_color: PieceColor,
    pub piece_type: PieceType,
    pub id: usize,
    pub position: Option<(usize, usize)>,
}
/// convenient builder to create chess pieces
struct CPB {
    current_id: usize,
    color: PieceColor,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

//==============================================================//
//  IMPLEMENTATIONS
//==============================================================//
impl ToString for ChessBoard {
    fn to_string(&self) -> String {
        let mut buffer = String::from("|--|--|--|--|--|--|--|--|\n");
        for row in &self.board {
            buffer += "|";
            for item in row {
                if let Some(pi) = item {
                    let piece = &self.pieces[*pi];
                    buffer += &format!("{}", piece);
                } else {
                    buffer += "  ";
                }
                buffer += "|"
            }
            buffer += "\n|--|--|--|--|--|--|--|--|\n";
        }
        buffer
    }
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut b = CPB::new();
        let mut cb = ChessBoard {
            pieces: [
                b.black().rook(),
                b.black().knight(),
                b.black().bishop(),
                b.black().queen(),
                b.black().king(),
                b.black().bishop(),
                b.black().knight(),
                b.black().rook(),
                b.black().pawn(),
                b.black().pawn(),
                b.black().pawn(),
                b.black().pawn(),
                b.black().pawn(),
                b.black().pawn(),
                b.black().pawn(),
                b.black().pawn(),
                b.white().pawn(),
                b.white().pawn(),
                b.white().pawn(),
                b.white().pawn(),
                b.white().pawn(),
                b.white().pawn(),
                b.white().pawn(),
                b.white().pawn(),
                b.white().rook(),
                b.white().knight(),
                b.white().bishop(),
                b.white().queen(),
                b.white().king(),
                b.white().bishop(),
                b.white().knight(),
                b.white().rook(),
            ],
            board: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
        };
        for i in 0..cb.pieces.len() {
            let y = cb.pieces[i].position.unwrap().1;
            let x = cb.pieces[i].position.unwrap().0;

            cb.board[y][x] = Some(i);
        }
        cb
    }
    pub fn make_move(&mut self, origin: (usize, usize), destination: (usize, usize)) {
        let piece = self.board[origin.1][origin.0];
        self.board[origin.1][origin.0] = None;
        let destpiece = self.board[destination.1][destination.0];
        /* super::log(&format!(
            "{:?}[{:?}]->{:?}[{:?}]",
            piece, destpiece, origin, destination
        )); */
        if let Some(pi) = piece {
            let p = self.pieces.get_mut(pi).unwrap();
            p.position = Some(destination);
            self.board[destination.1][destination.0] = Some(pi);
        }
        if let Some(dpi) = destpiece {
            let p = self.pieces.get_mut(dpi).unwrap();
            p.position = None;
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&ChessPiece> {
        let index = self.board[y][x];
        if let Some(i) = index {
            Some(&self.pieces[i])
        } else {
            None
        }
    }

    pub fn can_move(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> bool {
        self.get_possible_moves(x0, y0).contains(&(x1, y1))
    }
    pub fn get_possible_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let piece = self.get(x, y).unwrap();
        match piece.piece_type {
            PieceType::Pawn => self.get_pawn_moves(&piece, x, y),
            PieceType::Knight => self.get_knight_moves(&piece, x, y),
            PieceType::Bishop => self.get_bishop_moves(&piece, x, y),
            PieceType::Rook => self.get_rook_moves(&piece, x, y),
            PieceType::Queen => self.get_queen_moves(&piece, x, y),
            PieceType::King => self.get_king_moves(&piece, x, y),
        }
    }
    pub fn get_pawn_moves(&self, piece: &ChessPiece, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut position = vec![];
        match piece.piece_color {
            PieceColor::Black => {
                if y < 7 {
                    if self.get(x, y + 1).is_none() {
                        position.push((x, y + 1))
                    }
                    if x < 7 {
                        if let Some(p) = self.get(x + 1, y + 1) {
                            if p.piece_color != piece.piece_color {
                                position.push((x + 1, y + 1))
                            }
                        }
                    }
                    if x > 0 {
                        if let Some(p) = self.get(x - 1, y + 1) {
                            if p.piece_color != piece.piece_color {
                                position.push((x - 1, y + 1))
                            }
                        }
                    }
                }
            }
            PieceColor::White => {
                if y > 0 {
                    if self.get(x, y - 1).is_none() {
                        position.push((x, y - 1))
                    }
                    if x < 7 {
                        if let Some(p) = self.get(x + 1, y - 1) {
                            if p.piece_color != piece.piece_color {
                                position.push((x + 1, y - 1))
                            }
                        }
                    }
                    if x > 0 {
                        if let Some(p) = self.get(x - 1, y - 1) {
                            if p.piece_color != piece.piece_color {
                                position.push((x - 1, y - 1))
                            }
                        }
                    }
                }
            }
        }
        position
    }
    pub fn get_knight_moves(&self, piece: &ChessPiece, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        if self.is_position_valid(Some(x + 1), Some(y + 2), piece.piece_color) {
            ret.push((x + 1, y + 2))
        }
        if self.is_position_valid(Some(x + 1), y.checked_sub(2), piece.piece_color) {
            ret.push((x + 1, y - 2))
        }
        if self.is_position_valid(x.checked_sub(1), Some(y + 2), piece.piece_color) {
            ret.push((x - 1, y + 2))
        }
        if self.is_position_valid(x.checked_sub(1), y.checked_sub(2), piece.piece_color) {
            ret.push((x - 1, y - 2))
        }
        if self.is_position_valid(Some(x + 2), Some(y + 1), piece.piece_color) {
            ret.push((x + 2, y + 1))
        }
        if self.is_position_valid(Some(x + 2), y.checked_sub(1), piece.piece_color) {
            ret.push((x + 2, y - 1))
        }
        if self.is_position_valid(x.checked_sub(2), Some(y + 1), piece.piece_color) {
            ret.push((x - 2, y + 1))
        }
        if self.is_position_valid(x.checked_sub(2), y.checked_sub(1), piece.piece_color) {
            ret.push((x - 2, y - 1))
        }
        ret
    }
    pub fn get_bishop_moves(&self, piece: &ChessPiece, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        let mut cx = x;
        let mut cy = y;
        let mycolor = piece.piece_color;
        // BOTTOM_RIGHT
        while cy < 7 && cx < 7 {
            cy += 1;
            cx += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // TOP-LEFT
        while cy > 0 && cx > 0 {
            cy -= 1;
            cx -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // BOTTOM-LEFT
        while cy < 7 && cx > 0 {
            cy += 1;
            cx -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // TOP-RIGHT
        while cy > 0 && cx < 7 {
            cy -= 1;
            cx += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        ret
    }
    pub fn get_rook_moves(&self, piece: &ChessPiece, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        let mut cx = x;
        let mut cy = y;
        let mycolor = piece.piece_color;
        // BOTTOM
        while cy < 7 {
            cy += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // TOP
        while cy > 0 {
            cy -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // LEFT
        while cx > 0 {
            cx -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // RIGHT
        while cx < 7 {
            cx += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        ret
    }
    pub fn get_queen_moves(&self, piece: &ChessPiece, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        let mut cx = x;
        let mut cy = y;
        let mycolor = piece.piece_color;
        // BOTTOM
        while cy < 7 {
            cy += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // TOP
        while cy > 0 {
            cy -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // LEFT
        while cx > 0 {
            cx -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // RIGHT
        while cx < 7 {
            cx += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // BOTTOM_RIGHT
        while cy < 7 && cx < 7 {
            cy += 1;
            cx += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // TOP-LEFT
        while cy > 0 && cx > 0 {
            cy -= 1;
            cx -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // BOTTOM-LEFT
        while cy < 7 && cx > 0 {
            cy += 1;
            cx -= 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        cx = x;
        cy = y;
        // TOP-RIGHT
        while cy > 0 && cx < 7 {
            cy -= 1;
            cx += 1;
            match self.get(cx, cy) {
                None => ret.push((cx, cy)),
                Some(ChessPiece {
                    piece_color: PieceColor::Black,
                    ..
                }) => {
                    if mycolor == PieceColor::Black {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
                Some(ChessPiece {
                    piece_color: PieceColor::White,
                    ..
                }) => {
                    if mycolor == PieceColor::White {
                        break;
                    } else {
                        ret.push((cx, cy));
                        break;
                    }
                }
            }
        }
        ret
    }
    pub fn get_king_moves(&self, piece: &ChessPiece, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        if self.is_position_valid(Some(x), Some(y + 1), piece.piece_color) {
            ret.push((x, y + 1))
        }
        if self.is_position_valid(Some(x), y.checked_sub(1), piece.piece_color) {
            ret.push((x, y - 1))
        }
        if self.is_position_valid(Some(x + 1), Some(y), piece.piece_color) {
            ret.push((x + 1, y))
        }
        if self.is_position_valid(x.checked_sub(1), Some(y), piece.piece_color) {
            ret.push((x - 1, y))
        }
        if self.is_position_valid(Some(x + 1), Some(y + 1), piece.piece_color) {
            ret.push((x + 1, y + 1))
        }
        if self.is_position_valid(Some(x + 1), y.checked_sub(1), piece.piece_color) {
            ret.push((x + 1, y - 1))
        }
        if self.is_position_valid(x.checked_sub(1), Some(y + 1), piece.piece_color) {
            ret.push((x - 1, y + 1))
        }
        if self.is_position_valid(x.checked_sub(1), y.checked_sub(1), piece.piece_color) {
            ret.push((x - 1, y - 1))
        }
        ret
    }

    fn is_position_valid(&self, x: Option<usize>, y: Option<usize>, color: PieceColor) -> bool {
        if let (Some(xv), Some(yv)) = (x, y) {
            if xv >= 8 || yv >= 8 {
                return false;
            }
            if let Some(p) = self.get(xv, yv) {
                p.piece_color != color
            } else {
                true
            }
        } else {
            false
        }
    }
}

impl Default for ChessBoard {
    fn default() -> ChessBoard {
        ChessBoard::new()
    }
}

impl CPB {
    pub fn new() -> Self {
        Self {
            color: PieceColor::White,
            current_id: 0,
        }
    }
    pub fn white(&mut self) -> &mut Self {
        self.color = PieceColor::White;
        self
    }
    pub fn black(&mut self) -> &mut Self {
        self.color = PieceColor::Black;
        self
    }
    pub fn pawn(&mut self) -> ChessPiece {
        self.create_piece(PieceType::Pawn)
    }
    pub fn knight(&mut self) -> ChessPiece {
        self.create_piece(PieceType::Knight)
    }
    pub fn bishop(&mut self) -> ChessPiece {
        self.create_piece(PieceType::Bishop)
    }
    pub fn rook(&mut self) -> ChessPiece {
        self.create_piece(PieceType::Rook)
    }
    pub fn queen(&mut self) -> ChessPiece {
        self.create_piece(PieceType::Queen)
    }
    pub fn king(&mut self) -> ChessPiece {
        self.create_piece(PieceType::King)
    }
    pub fn create_piece(&mut self, piece_type: PieceType) -> ChessPiece {
        self.current_id += 1;
        let mut row = (self.current_id - 1) / 8;
        if row > 1 {
            row += 4;
        }
        ChessPiece {
            piece_color: self.color,
            piece_type,
            id: self.current_id,
            position: Some(((self.current_id - 1) % 8, row)),
        }
    }
}
impl ChessPiece {}
impl std::fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.piece_color {
                PieceColor::Black => "B",
                PieceColor::White => "W",
            },
            match self.piece_type {
                PieceType::Pawn => "P",
                PieceType::Knight => "k",
                PieceType::Bishop => "B",
                PieceType::Rook => "R",
                PieceType::Queen => "Q",
                PieceType::King => "K",
            }
        )
    }
}

//==============================================================//
//  TESTS
//==============================================================//

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board_creation() {
        let chessboard = ChessBoard::new();
        // KING
        let bk = &(chessboard.get(4, 0).unwrap());
        assert!(bk.piece_color == PieceColor::Black);
        assert!(bk.piece_type == PieceType::King);
        assert!(Some((4, 0)) == bk.position);
        // PAWN
        let bp = &(chessboard.get(4, 1).unwrap());
        assert!(bp.piece_color == PieceColor::Black);
        assert!(bp.piece_type == PieceType::Pawn);
        assert!(Some((4, 1)) == bp.position);
    }
    #[test]

    fn empty_space_should_not_panic() {
        let chessboard = ChessBoard::new();
        let empty = chessboard.get(1, 4);
        assert!(empty.is_none());
    }
    #[test]
    fn move_pawn() {
        let mut chessboard = ChessBoard::new();
        chessboard.make_move((4, 1), (4, 2));

        let bp = &(chessboard.get(4, 2).unwrap());
        assert_eq!(bp.piece_color, PieceColor::Black);
        assert_eq!(bp.piece_type, PieceType::Pawn);
        assert_eq!(Some((4, 2)), bp.position);
    }
}

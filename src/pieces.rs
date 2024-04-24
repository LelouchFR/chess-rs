#[derive(Default, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    #[default]
    None,
}

#[derive(Default, Clone, PartialEq)]
pub enum Color {
    Black,
    White,
    #[default]
    None,
}

#[derive(Clone, PartialEq)]
pub struct Pieces {
    pub piece: Piece,
    pub color: Color,
}

impl Pieces {
    pub fn new(piece: Piece, color: Color) -> Pieces {
        Pieces { piece, color }
    }

    pub fn default() -> Pieces {
        Pieces {
            piece: Piece::None,
            color: Color::None,
        }
    } 

    pub fn render(self: &Self) -> &'static str {
        match (&self.piece, &self.color) {
            (Piece::Pawn, Color::Black) => "♙",
            (Piece::Pawn, Color::White) => "♟",
            (Piece::Bishop, Color::Black) => "♗",
            (Piece::Bishop, Color::White) => "♝",
            (Piece::Knight, Color::Black) => "♘",
            (Piece::Knight, Color::White) => "♞",
            (Piece::Rook, Color::Black) => "♖",
            (Piece::Rook, Color::White) => "♜",
            (Piece::Queen, Color::Black) => "♕",
            (Piece::Queen, Color::White) => "♛",
            (Piece::King, Color::Black) => "♔",
            (Piece::King, Color::White) => "♚",
            (_, _) => "",
        }
    }
}

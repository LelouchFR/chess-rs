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

    pub fn render(self: Self) -> String {
        match (self.piece, self.color) {
            (Piece::Pawn, Color::Black) => "♙".to_string(),
            (Piece::Pawn, Color::White) => "♟".to_string(),
            (Piece::Bishop, Color::Black) => "♗".to_string(),
            (Piece::Bishop, Color::White) => "♝".to_string(),
            (Piece::Knight, Color::Black) => "♘".to_string(),
            (Piece::Knight, Color::White) => "♞".to_string(),
            (Piece::Rook, Color::Black) => "♖".to_string(),
            (Piece::Rook, Color::White) => "♜".to_string(),
            (Piece::Queen, Color::Black) => "♕".to_string(),
            (Piece::Queen, Color::White) => "♛".to_string(),
            (Piece::King, Color::Black) => "♔".to_string(),
            (Piece::King, Color::White) => "♚".to_string(),
            (_, _) => "".to_string(),
        }
    }
}

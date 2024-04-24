use crate::pieces::{Color, Piece, Pieces};
pub struct Board {
    pub board: Vec<Vec<Option<Pieces>>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![
                vec![
                    Some(Pieces::new(Piece::Rook, Color::Black)),
                    Some(Pieces::new(Piece::Knight, Color::Black)),
                    Some(Pieces::new(Piece::Bishop, Color::Black)),
                    Some(Pieces::new(Piece::Queen, Color::Black)),
                    Some(Pieces::new(Piece::King, Color::Black)),
                    Some(Pieces::new(Piece::Bishop, Color::Black)),
                    Some(Pieces::new(Piece::Knight, Color::Black)),
                    Some(Pieces::new(Piece::Rook, Color::Black)),
                ],
                vec![Some(Pieces::new(Piece::Pawn, Color::Black)); 8],
                vec![None; 8],
                vec![None; 8],
                vec![None; 8],
                vec![None; 8],
                vec![Some(Pieces::new(Piece::Pawn, Color::White));8],
                vec![
                    Some(Pieces::new(Piece::Rook, Color::White)),
                    Some(Pieces::new(Piece::Knight, Color::White)),
                    Some(Pieces::new(Piece::Bishop, Color::White)),
                    Some(Pieces::new(Piece::Queen, Color::White)),
                    Some(Pieces::new(Piece::King, Color::White)),
                    Some(Pieces::new(Piece::Bishop, Color::White)),
                    Some(Pieces::new(Piece::Knight, Color::White)),
                    Some(Pieces::new(Piece::Rook, Color::White)),
                ],
            ],
        }
    }

    pub fn render(&self) -> String {
        let mut result: String = String::new();

        for line in &self.board {
            for square in line {
                if let Some(piece) = square {
                    result += &piece.render();
                } else {
                    result += " ";
                }
                result += " ";
            }
            result += "\n";
        }
        result
    }

    pub fn get_piece(&self, from: (usize, usize)) -> Option<Pieces> {
        let (row, col) = from;
        match &self.board[row][col] {
            Some(piece) => Some(piece.clone()),
            None => None,
        }
    }

    pub fn set_piece(&mut self, from: (usize, usize), piece: Option<Pieces>) {
        let (row, col) = from;
        self.board[row][col] = piece;
    }
}

use crate::pieces::{Color, Piece, Pieces};
pub struct Board {
    pub board: Vec<Vec<Pieces>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![
                vec![
                    Pieces::new(Piece::Rook, Color::Black),
                    Pieces::new(Piece::Knight, Color::Black),
                    Pieces::new(Piece::Bishop, Color::Black),
                    Pieces::new(Piece::Queen, Color::Black),
                    Pieces::new(Piece::King, Color::Black),
                    Pieces::new(Piece::Bishop, Color::Black),
                    Pieces::new(Piece::Knight, Color::Black),
                    Pieces::new(Piece::Rook, Color::Black),
                ],
                vec![Pieces::new(Piece::Pawn, Color::Black); 8],
                vec![Pieces::default(); 8],
                vec![Pieces::default(); 8],
                vec![Pieces::default(); 8],
                vec![Pieces::default(); 8],
                vec![Pieces::new(Piece::Pawn, Color::White);8],
                vec![
                    Pieces::new(Piece::Rook, Color::White),
                    Pieces::new(Piece::Knight, Color::White),
                    Pieces::new(Piece::Bishop, Color::White),
                    Pieces::new(Piece::Queen, Color::White),
                    Pieces::new(Piece::King, Color::White),
                    Pieces::new(Piece::Bishop, Color::White),
                    Pieces::new(Piece::Knight, Color::White),
                    Pieces::new(Piece::Rook, Color::White),
                ],
            ],
        }
    }

    pub fn render(&self) -> String {
        let mut result: String = String::new();

        for line in &self.board {
            result += &format!(
                "{} {} {} {} {} {} {} {}\n",
                line[0].clone().render(),
                line[1].clone().render(),
                line[2].clone().render(),
                line[3].clone().render(),
                line[4].clone().render(),
                line[5].clone().render(),
                line[6].clone().render(),
                line[7].clone().render(),
            )
            .to_string();
        }
        result
    }

    pub fn get_piece(&self, from: (usize, usize)) -> Pieces {
        let (row, col) = from;
        self.board[row][col].clone()
    }

    pub fn set_piece(&mut self, from: (usize, usize), piece: Pieces) {
        let (row, col) = from;
        self.board[row][col] = piece;
    }
}

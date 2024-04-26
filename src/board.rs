use crate::{
    engine::{Engine, Player},
    pieces::{Color, Piece, Pieces},
    utils::Position,
};

#[derive(Default, Clone)]
pub struct Board {
    pub board: Vec<Vec<Option<Pieces>>>,
}

impl Board {
    pub fn default() -> Board {
        Board {
            board: vec![vec![None; 8]; 8],
        }
    }

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
                vec![Some(Pieces::new(Piece::Pawn, Color::White)); 8],
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

    pub fn create(&self, custom_board: Vec<Vec<Option<i8>>>) -> Board {
        let mut new_custom_board = Vec::new();

        for row in custom_board {
            let mut new_row = Vec::new();
            for col in row {
                let new_piece = match col {
                    Some(1) => Some(Pieces::new(Piece::Pawn, Color::White)),
                    Some(-1) => Some(Pieces::new(Piece::Pawn, Color::Black)),
                    Some(2) => Some(Pieces::new(Piece::Bishop, Color::White)),
                    Some(-2) => Some(Pieces::new(Piece::Bishop, Color::Black)),
                    Some(3) => Some(Pieces::new(Piece::Knight, Color::White)),
                    Some(-3) => Some(Pieces::new(Piece::Knight, Color::Black)),
                    Some(4) => Some(Pieces::new(Piece::Rook, Color::White)),
                    Some(-4) => Some(Pieces::new(Piece::Rook, Color::Black)),
                    Some(5) => Some(Pieces::new(Piece::Queen, Color::White)),
                    Some(-5) => Some(Pieces::new(Piece::Queen, Color::Black)),
                    Some(6) => Some(Pieces::new(Piece::King, Color::White)),
                    Some(-6) => Some(Pieces::new(Piece::King, Color::Black)),
                    _ => None,
                };
                new_row.push(new_piece);
            }
            new_custom_board.push(new_row);
        }

        Board {
            board: new_custom_board,
        }
    }

    pub fn scan(&self, engine: &Engine) -> Vec<Vec<Position>> {
        let mut all_possible_moves = Vec::new();

        for row in 0..=self.board.len() - 1 {
            for col in 0..=row {
                if let Some(piece) = &engine.state.board[row][col] {
                    all_possible_moves
                        .push(engine.get_valid_moves((row, col), piece.piece.clone()));
                }
            }
        }

        all_possible_moves
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

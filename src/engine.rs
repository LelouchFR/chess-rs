use crate::{
    board::Board,
    pieces::{Piece, Pieces},
    utils::{Position, get_valid_position},
};

#[derive(PartialEq)]
pub enum Player {
    White,
    Black,
}

pub struct Engine {
    pub state: Board,
    pub current_player: Player,
}

impl Engine {
    pub fn new(state: Board, current_player: Player) -> Engine {
        Engine {
            state,
            current_player,
        }
    }

    pub fn make_move(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        if !self.is_valid_move(from, to, self.state.get_piece(from).unwrap().piece) {
            return Err("Invalid move");
        }

        let piece = self.state.get_piece(from).unwrap();
        self.state.set_piece(from, None);
        self.state.set_piece(to, Some(piece));

        self.current_player = match self.current_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        Ok(())
    }

    // TODO at the end
    pub fn evaluate(&self) -> f32 {
        0.0
    }
}

// get_xxx
impl Engine {
    pub fn get_valid_moves(&self, from: Position, piece: Piece) -> Vec<Position> {
        match piece {
            Piece::Pawn => self.get_valid_pawn_move(from),
            Piece::Bishop => self.get_valid_bishop_move(from),
            Piece::Knight => self.get_valid_knight_move(from),
            Piece::Rook => self.get_valid_rook_move(from),
            Piece::Queen => self.get_valid_queen_move(from),
            Piece::King => self.get_valid_king_move(from),
        }
    }

    fn get_valid_pawn_move(&self, from: Position) -> Vec<Position> {
        let (row, col) = from;
        let mut valid_moves = Vec::new();

        let direction = match self.current_player {
            Player::White => -1,
            Player::Black => 1,
        };

        let forward_one = ((row as isize) + direction, col);
        if forward_one.0 >= 0
            && forward_one.0 < 8
            && self.is_valid_move(from, (forward_one.0 as usize, forward_one.1), Piece::Pawn)
        {
            valid_moves.push((forward_one.0 as usize, forward_one.1));
        }

        let starting_row = match self.current_player {
            Player::White => 6,
            Player::Black => 1,
        };

        let forward_two = (((row as isize) + 2 * direction), col);
        if starting_row == row 
            && self.is_valid_move(from, (forward_two.0 as usize, forward_two.1), Piece::Pawn)
        {
                valid_moves.push((forward_two.0 as usize, forward_two.1));
        }

        let captures = &[
            (((row as isize) + direction), (col as isize) + 1),
            (((row as isize) + direction), (col as isize) - 1),
        ];
        for &(r, c) in captures {
            if r >= 0 && r < 8 && c >= 0 && c < 8 {
                let capture_pos = (r as usize, c as usize);
                if let Some(_) = self.state.get_piece(capture_pos) {
                    valid_moves.push(capture_pos);
                }
            }
        }

        valid_moves
    }

    fn get_valid_bishop_move(&self, from: Position) -> Vec<Position> {
        let (row, col) = from;
        let mut valid_moves = Vec::new();
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        for (direction_row, direction_col) in directions.iter() {
            let mut current_row = row as isize + *direction_row as isize;
            let mut current_col = col as isize + *direction_col as isize;

            while current_row >= 0 && current_row < 8 && current_col >= 0 && current_col < 8 {
                let valid_position = get_valid_position((current_row, current_col));
                match valid_position {
                    Some((r, c)) => {
                        if let Some(_) = self.state.get_piece((r, c)) {
                            if !matches!(self.state.get_piece((r, c)).unwrap(), Piece) {
                                valid_moves.push((r, c));
                            }
                            break;
                        } else {
                            valid_moves.push((r, c));
                        }
                    },
                    None => break,
                }
                current_row += *direction_row as isize;
                current_col += *direction_col as isize;
            }
        }

        valid_moves
    }

    fn get_valid_knight_move(&self, from: Position) -> Vec<Position> {
        let (row, col) = from;
        let mut valid_moves = Vec::new();
        let directions = [(-2, 1), (-2, -1), (2, 1), (2, -1)];

        for (direction_row, direction_col) in directions.iter() {
            let new_row = row as isize + *direction_row as isize;
            let new_col = col as isize + *direction_col as isize;

            if let Some((r, c)) = get_valid_position((new_row, new_col)) {
                if let Some(_) = self.state.get_piece((r, c)) {
                    valid_moves.push((r, c));
                } else {
                    valid_moves.push((r, c));
                }
            }
        }

        valid_moves
    }

    fn get_valid_rook_move(&self, from: Position) -> Vec<Position> {
        vec![]
    }

    fn get_valid_queen_move(&self, from: Position) -> Vec<Position> {
        vec![]
    }

    fn get_valid_king_move(&self, from: Position) -> Vec<Position> {
        vec![]
    }
}

// is_xxx
impl Engine {
    pub fn is_valid_move(&self, from: Position, to: Position, piece: Piece) -> bool {
        match piece {
            Piece::Pawn => self.is_valid_pawn_move(from, to),
            Piece::Bishop => self.is_valid_bishop_move(from, to),
            Piece::Knight => self.is_valid_knight_move(from, to),
            Piece::Rook => self.is_valid_rook_move(from, to),
            Piece::Queen => self.is_valid_queen_move(from, to),
            Piece::King => self.is_valid_king_move(from, to),
        }
    }

    fn is_valid_pawn_move(&self, from: Position, to: Position) -> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        let direction = match self.current_player {
            Player::White => -1,
            Player::Black => 1,
        };

        let forward_one = ((from_row as isize) + direction, from_col);
        if forward_one.0 as usize == to_row
            && forward_one.1 == to_col
            && self.state.get_piece(to).is_none()
        {
            return true;
        }

        let starting_row = match self.current_player {
            Player::White => 6,
            Player::Black => 1,
        };

        let forward_two = (from_row as isize + 2 * direction) as usize;
        if forward_two == to_row
            && from_row == starting_row
            && self
                .state
                .get_piece((forward_one.0 as usize, forward_one.1))
                .is_none()
            && self.state.get_piece(to).is_none()
        {
            return true;
        }

        let captures = &[
            get_valid_position((from_row as isize + direction, (from_col as isize + 1) as isize)),
            get_valid_position((from_row as isize + direction, (from_col as isize - 1) as isize)),
        ];
        for &capture in captures {
            if let Some((capture_row, capture_col)) = capture {
                if let Some(_) = self.state.get_piece((capture_row, capture_col)) {
                    if capture_row == to_row && capture_col == to_col {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    fn is_valid_bishop_move(&self, from: Position, to: Position) -> bool {
        let valid_moves = self.get_valid_moves(from, Piece::Bishop);
        valid_moves.contains(&to)
    }

    fn is_valid_knight_move(&self, from: Position, to: Position) -> bool {
        let valid_moves = self.get_valid_moves(from, Piece::Knight);
        valid_moves.contains(&to)
    }

    fn is_valid_rook_move(&self, from: Position, to: Position) -> bool {
        true
    }
    
    fn is_valid_queen_move(&self, from: Position, to: Position) -> bool {
        true
    }

    fn is_valid_king_move(&self, from: Position, to: Position) -> bool {
        true
    }
}

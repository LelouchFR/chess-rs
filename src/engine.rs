use crate::{
    board::Board,
    pieces::{Piece, Pieces},
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

type Position = (usize, usize);

impl Engine {
    pub fn new(state: Board, current_player: Player) -> Engine {
        Engine {
            state,
            current_player,
        }
    }

    pub fn make_move(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        if !self.is_valid_move(from, to) {
            return Err("Invalid move");
        }

        let piece = self.state.get_piece(from);
        self.state.set_piece(from, Pieces::default());
        self.state.set_piece(to, piece);

        self.current_player = match self.current_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        Ok(())
    }

    pub fn get_valid_moves(&self, from: Position, piece: Piece) -> Vec<Position> {
        match piece {
            Piece::Pawn => self.get_valid_pawn_move(from),
            _ => vec![],
        }
    }

    pub fn get_valid_pawn_move(&self, from: Position) -> Vec<Position> {
        let (row, col) = from;
        let mut valid_moves = Vec::new();

        let direction = match self.current_player {
            Player::White => -1,
            Player::Black => 1,
        };

        let forward_one = ((row as isize) + direction, col);
        if forward_one.0 >= 0
            && forward_one.0 < 8
            && self.is_valid_move(from, (forward_one.0 as usize, forward_one.1))
        {
            valid_moves.push((forward_one.0 as usize, forward_one.1));
        }

        let forward_two = (((row as isize) + 2 * direction), col);
        if ((self.current_player == Player::White && row == 6)
            || (self.current_player == Player::Black && row == 1))
            && self.is_valid_move(from, (forward_two.0 as usize, forward_two.1))
        {
            if self.is_valid_move(from, (forward_two.0 as usize, forward_two.1)) {
                valid_moves.push((forward_two.0 as usize, forward_two.1));
            }
        }

        let captures = &[
            (((row as isize) + direction), (col as isize) + 1),
            (((row as isize) + direction), (col as isize) - 1),
        ];
        for &(r, c) in captures {
            if r >= 0 && r < 8 && c >= 0 && c < 8 {
                let capture_pos = (r as usize, c as usize);
                if self.state.board[capture_pos.0][capture_pos.1].piece != Piece::None {
                    valid_moves.push(capture_pos);
                }
            }
        }

        valid_moves
    }

    // TODO
    fn is_valid_move(&self, from: Position, to: Position) -> bool {
        true
    }

    // TODO
    pub fn evaluate(&self) -> i32 {
        0
    }
}

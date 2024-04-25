// mod app;
mod board;
mod engine;
mod pieces;

use crate::board::Board;
use crate::engine::{Engine, Player};
use crate::pieces::Piece;

// use crate::{app::app};
use std::io;
// use std::io::{self, stdout};
// use crossterm::{
//     event::{self, Event, KeyCode},
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//     ExecutableCommand,
// };
// use ratatui::{prelude::*, widgets::*};

// fn handle_events() -> io::Result<bool> {
//     if event::poll(std::time::Duration::from_millis(50))? {
//         if let Event::Key(key) = event::read()? {
//             if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                 return Ok(true);
//             }
//         }
//     }
//
//     Ok(false)
// }

fn main() -> io::Result<()> {
    // enable_raw_mode()?;
    // stdout().execute(EnterAlternateScreen)?;
    // let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    //
    // let mut should_quit: bool = false;
    // while !should_quit {
    //     terminal.draw(app)?;
    //     should_quit = handle_events()?;
    // }
    //
    // disable_raw_mode()?;
    // stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}


#[cfg(test)]
#[test]
fn eating_pawn() {
    let board = Board::new();
    let mut engine = Engine::new(board, Player::White);
    println!("{}", engine.state.render());
    println!(
        "possible moves for (6, 0) (white move): {:?}",
        engine.get_valid_moves((6, 0), Piece::Pawn)
    );
    let _ = engine.make_move((6, 0), engine.get_valid_moves((6, 0), Piece::Pawn)[1]); // move pawn from 1up
    println!("{}", engine.state.render());
    println!(
        "possible moves for (1, 3) (black move): {:?}",
        engine.get_valid_moves((1, 1), Piece::Pawn)
    );
    let _ = engine.make_move((1, 1), engine.get_valid_moves((1, 1), Piece::Pawn)[1]); // move pawn from 1up
    println!("{}", engine.state.render());
    println!(
        "possible moves for (4, 0): {:?}",
        engine.get_valid_moves((4, 0), Piece::Pawn)
    );
    // Error: attempt to substract with overflow
    let _ = engine.make_move((4, 0), engine.get_valid_moves((4, 0), Piece::Pawn)[1]); // eating_pawn
    println!("{}", engine.state.render());
    println!(
        "possible moves for (4, 0): {:?}",
        engine.get_valid_moves((4, 0), Piece::Pawn)
    );
}

#[test]
fn simple_game() {
    let board = Board::new();
    let mut engine = Engine::new(board, Player::White);
    println!("{}", engine.state.render());
    println!(
        "possible moves for (6, 0) (white move): {:?}",
        engine.get_valid_moves((6, 0), Piece::Pawn)
    );
    let _ = engine.make_move((6, 0), engine.get_valid_moves((6, 0), Piece::Pawn)[1]); // move pawn from 1up
    println!("{}", engine.state.render());
    println!(
        "possible moves for (1, 3) (black move): {:?}",
        engine.get_valid_moves((1, 1), Piece::Pawn)
    );
    // Error: attempt to substract with overflow
    let _ = engine.make_move((1, 1), engine.get_valid_moves((1, 1), Piece::Pawn)[1]); // eating_pawn
    println!("{}", engine.state.render());
    println!(
        "possible moves for (4, 0): {:?}",
        engine.get_valid_moves((4, 0), Piece::Pawn)
    );
}

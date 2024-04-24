use ratatui::{prelude::*, widgets::*};
use crate::board::Board;

pub fn app(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("X")
            .block(Block::default().borders(Borders::ALL)),
        frame.size(),
   );
}

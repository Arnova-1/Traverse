use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Widget}};

use crate::state::AppState;

impl Widget for &AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Traverse ".bold());
        let instructions = Line::from(vec![
            " Previous ".into(),
            "<Left>".blue().bold(),
            " Next ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::PLAIN);
    }
}

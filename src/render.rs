use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, List, ListItem, Widget}};

use crate::state::{AppState, FileState};

impl Widget for &AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Traverse ".bold());
        let instructions = Line::from(vec![
            " Go Up ".into(),
            "<Up>".blue().bold(),
            " Previous ".into(),
            "<Left>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
            " Next ".into(),
            "<Right>".blue().bold(),
            " Go Down ".into(),
            "<Down> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::PLAIN);

        let files = FileState::formatted_lines(&mut FileState::default());
        let items: Vec<ListItem> = files
            .iter()
            .map(|s| ListItem::new(s.as_str()))
            .collect();
        let item_list = List::new(items);

        List::from(item_list)
            .block(block)
            .render(area, buf);
    }
}

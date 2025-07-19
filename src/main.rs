use std::io;

use crate::state::AppState;

mod state;
mod fs;
mod ui;
mod render;
mod handle_key;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = AppState::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

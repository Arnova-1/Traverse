use std::cmp::min;

use crossterm::event::{KeyCode, KeyEvent};

use crate::state::{AppState, FileState};

impl AppState {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.path_backward(),
            KeyCode::Right => self.path_forward(),
            KeyCode::Up => self.go_up(),
            KeyCode::Down => self.go_down(&FileState::default()),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn path_forward(&mut self) {
        todo!()
    }

    fn path_backward(&mut self) {
        todo!()
    }

    fn go_up(&mut self) {
        let selected_index = self.selected.saturating_sub(1);
        self.selected = selected_index
    }

    fn go_down(&mut self, files: &FileState) {
        let selected_index = min(self.selected + 1, files.files.len() - 1);
        self.selected = selected_index
    }

}

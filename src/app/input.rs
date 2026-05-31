use crate::app::App;

impl App {
    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    pub fn delete_before(&mut self) {
        if self.cursor > 0 {
            let prev = self.input[..self.cursor].char_indices().next_back();
            if let Some((idx, _)) = prev {
                self.input.drain(idx..self.cursor);
                self.cursor = idx;
            }
        }
    }

    pub fn delete_at(&mut self) {
        if self.cursor < self.input.len() {
            let next = self.input[self.cursor..].char_indices().nth(1);
            let end = next
                .map(|(i, _)| self.cursor + i)
                .unwrap_or(self.input.len());
            self.input.drain(self.cursor..end);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            let prev = self.input[..self.cursor].char_indices().next_back();
            if let Some((idx, _)) = prev {
                self.cursor = idx;
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.input.len() {
            let next = self.input[self.cursor..].char_indices().nth(1);
            if let Some((idx, c)) = next {
                self.cursor = self.cursor + idx + c.len_utf8();
            }
        }
    }

    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor = self.input.len();
    }

    pub fn history_back(&mut self) {
        if self.history.is_empty() {
            return;
        }
        match self.history_pos {
            None => {
                self.history_pos = Some(self.history.len() - 1);
                self.input = self.history[self.history.len() - 1].clone();
            }
            Some(pos) if pos > 0 => {
                self.history_pos = Some(pos - 1);
                self.input = self.history[pos - 1].clone();
            }
            _ => {}
        }
        self.cursor = self.input.len();
    }

    pub fn history_forward(&mut self) {
        match self.history_pos {
            Some(pos) if pos < self.history.len() - 1 => {
                self.history_pos = Some(pos + 1);
                self.input = self.history[pos + 1].clone();
            }
            _ => {
                self.history_pos = None;
                self.input.clear();
            }
        }
        self.cursor = self.input.len();
    }

    pub fn scroll_results_up(&mut self, page_size: usize) {
        if let Some(offset) = self.block_row_scroll.get_mut(self.active_block) {
            *offset = offset.saturating_sub(page_size);
        }
    }

    pub fn scroll_results_down(&mut self, page_size: usize) {
        if let Some(offset) = self.block_row_scroll.get_mut(self.active_block) {
            *offset = offset.saturating_add(page_size);
        }
    }

    pub fn scroll_x_left(&mut self) {
        self.scroll_x = self.scroll_x.saturating_sub(8);
    }

    pub fn scroll_x_right(&mut self) {
        self.scroll_x = self.scroll_x.saturating_add(8);
    }
}

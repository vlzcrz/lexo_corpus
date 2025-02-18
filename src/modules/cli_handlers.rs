use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

use crossterm::event::{poll, read, Event, KeyCode};
use std::{io, time::Duration};
use tui::Terminal;

mod utils;

use utils::terminal_utils::{create_primary_block, create_terminal_backend, restore_terminal};

fn main() -> Result<(), io::Error> {
  if let Ok(backend) = create_terminal_backend() {
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
      let size = f.size();
      let block = create_primary_block("Redis TUI");
      f.render_widget(block, size);
    })?;

    loop {
      if poll(Duration::from_millis(500))? {
        match read()? {
          Event::Key(event) => {
            if event.code == KeyCode::Char('q') {
              if let Ok(_) = restore_terminal(&mut terminal) {
                return Ok(());
              } else {
                panic!("Failed to restore terminal");
              }
            } else {
              continue;
            }
          }
          _ => continue,
        }
      }
    }
  } else {
    Err(io::Error::new(
      io::ErrorKind::Other,
      "Failed to create backend",
    ))
  }
}

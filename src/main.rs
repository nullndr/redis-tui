mod utils;

use crossterm::event::{self, Event, KeyCode};
use std::{
  io::{Error, ErrorKind},
  time::Duration,
};
use tui::Terminal;
use utils::terminal_utils;

fn main() -> Result<(), Error> {
  if let Ok(backend) = terminal_utils::create_terminal_backend() {
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
      let size = f.size();
      let block = terminal_utils::create_primary_block("Redis TUI");
      f.render_widget(block, size);
    })?;

    loop {
      if event::poll(Duration::from_millis(500))? {
        match event::read()? {
          Event::Key(event) => {
            if event.code == KeyCode::Char('q') {
              if let Ok(_) = terminal_utils::restore_terminal(&mut terminal) {
                break Ok(());
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
    Err(Error::new(
      ErrorKind::Other,
      "Failed to create terminal backend",
    ))
  }
}

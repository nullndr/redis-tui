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

    loop {
      terminal.autoresize()?;
      terminal.draw(|f| {
        let size = f.size();
        let block = terminal_utils::create_block("Redis TUI");
        f.render_widget(block, size);
      })?;
      if event::poll(Duration::from_millis(500))? {
        match event::read()? {
          Event::Key(event) => {
            if event.code == KeyCode::Char('q') {
              terminal_utils::close_application(&mut terminal)?;
              return Ok(());
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

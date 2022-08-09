mod events;
mod utils;

use crossterm::event::{self, Event, KeyCode};
use std::{io::Error, time::Duration};
use utils::terminal_utils;

fn main() -> Result<(), Error> {
  let mut terminal = terminal_utils::create_terminal()?;

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
}

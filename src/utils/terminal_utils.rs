use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};
use tui::{
  backend::CrosstermBackend,
  style::{Color, Modifier, Style},
  text::Span,
  widgets::{Block, Borders},
  Terminal,
};

pub type TerminalBackendTp = CrosstermBackend<io::Stdout>;
pub type TerminalBackend<W> = CrosstermBackend<W>;

pub fn create_terminal_backend() -> Result<TerminalBackendTp, io::Error> {
  terminal::enable_raw_mode()?;
  let mut stdout = stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  Ok(TerminalBackend::new(stdout))
}

pub fn restore_terminal(terminal: &mut Terminal<TerminalBackendTp>) -> Result<(), io::Error> {
  terminal::disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  Ok(())
}

pub fn create_primary_block(title: &str) -> Block<'static> {
  Block::default()
    .title(Span::styled(
      String::from(title),
      Style::default().add_modifier(Modifier::BOLD),
    ))
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::White))
}

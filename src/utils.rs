pub mod utils {
  use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  };
  use std::io::{stdout, Stdout};
  use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
  };

  pub fn create_backend() -> Result<CrosstermBackend<Stdout>, ()> {
    if let Ok(_) = enable_raw_mode() {
      let mut stdout = stdout();
      if let Ok(_) = execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
        Ok(CrosstermBackend::new(stdout))
      } else {
        Err(())
      }
    } else {
      Err(())
    }
  }

  pub fn restore_terminal(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), ()> {
    if let Ok(_) = disable_raw_mode() {
      if let Ok(_) = execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
      ) {
        Ok(())
      } else {
        Err(())
      }
    } else {
      Err(())
    }
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
}

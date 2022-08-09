use crossterm::event::{self, KeyCode};
use std::{sync::mpsc, thread, time::Duration};

pub enum Event<E> {
  Input(E),
  Tick,
}

pub struct Events {
  rx: mpsc::Receiver<Event<KeyCode>>,
  tx: mpsc::Sender<Event<KeyCode>>,
}

pub struct EventConfig {
  pub tick_rate: Duration,
  pub exit_key: KeyCode,
}

impl Default for EventConfig {
  fn default() -> Self {
    EventConfig {
      tick_rate: Duration::from_millis(200),
      exit_key: KeyCode::Char('q'),
    }
  }
}

impl Events {
  pub fn new(tick_rate: u64) -> Events {
    Events::config(EventConfig {
      tick_rate: Duration::from_millis(tick_rate),
      ..Default::default()
    })
  }

  pub fn config(cfg: EventConfig) -> Events {
    let (tx, rx) = mpsc::channel();
    let event_tx = tx.clone();
    thread::spawn(move || loop {
      if event::poll(cfg.tick_rate).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
          let key = KeyCode::from(key.code);
          event_tx.send(Event::Input(key)).unwrap();
        }
      }
      event_tx.send(Event::Tick).unwrap();
    });

    Events { rx, tx }
  }

  pub fn next(&self) -> Result<Event<KeyCode>, mpsc::RecvError> {
    self.rx.recv()
  }
}

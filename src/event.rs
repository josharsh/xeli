use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum AppEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    #[allow(dead_code)]
    Tick,
    AiResponse(String),
    AiError(String),
    #[allow(dead_code)]
    AiDone,
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<AppEvent>,
    _tx: mpsc::UnboundedSender<AppEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let event_tx = tx.clone();

        tokio::spawn(async move {
            loop {
                if event::poll(tick_rate).unwrap_or(false) {
                    match event::read() {
                        Ok(CrosstermEvent::Key(key)) => {
                            if event_tx.send(AppEvent::Key(key)).is_err() {
                                return;
                            }
                        }
                        Ok(CrosstermEvent::Mouse(mouse)) => {
                            if event_tx.send(AppEvent::Mouse(mouse)).is_err() {
                                return;
                            }
                        }
                        Ok(CrosstermEvent::Resize(w, h)) => {
                            if event_tx.send(AppEvent::Resize(w, h)).is_err() {
                                return;
                            }
                        }
                        _ => {}
                    }
                }
            }
        });

        Self { rx, _tx: tx }
    }

    pub async fn next(&mut self) -> Option<AppEvent> {
        self.rx.recv().await
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<AppEvent> {
        self._tx.clone()
    }
}

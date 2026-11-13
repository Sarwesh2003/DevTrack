use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct AppState {
  pub initialized: AtomicBool,
}

impl AppState {
  pub fn mark_initialized(&self) {
    self.initialized.store(true, Ordering::SeqCst);
  }

  pub fn is_initialized(&self) -> bool {
    self.initialized.load(Ordering::SeqCst)
  }
}

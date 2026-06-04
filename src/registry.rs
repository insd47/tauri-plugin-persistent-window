use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};

/// Single source of truth for window persistence.
///
/// Holds which windows survive a close and the order in which they were hidden.
/// Every other module reads or mutates persistence through this type, injected
/// as Tauri managed state — so policy (`lifecycle`), restoration (`reopen`) and
/// the public API (`ext`) never touch each other's internals.
#[derive(Default)]
pub struct Registry(Mutex<Inner>);

#[derive(Default)]
struct Inner {
  /// Labels whose close hides the window instead of destroying it.
  persistent: HashSet<String>,
  /// Hidden persistent labels, most-recently-hidden first.
  hidden: Vec<String>,
}

impl Registry {
  pub fn set(&self, label: &str, persistent: bool) {
    let mut inner = self.lock();
    if persistent {
      inner.persistent.insert(label.to_owned());
    } else {
      inner.persistent.remove(label);
      inner.hidden.retain(|l| l != label);
    }
  }

  pub fn is_persistent(&self, label: &str) -> bool {
    self.lock().persistent.contains(label)
  }

  /// Record `label` as the most recently hidden window.
  pub fn mark_hidden(&self, label: &str) {
    let mut inner = self.lock();
    inner.hidden.retain(|l| l != label);
    inner.hidden.insert(0, label.to_owned());
  }

  /// The most recently hidden persistent window, if any.
  pub fn last_hidden(&self) -> Option<String> {
    self.lock().hidden.first().cloned()
  }

  pub fn forget_hidden(&self, label: &str) {
    self.lock().hidden.retain(|l| l != label);
  }

  fn lock(&self) -> MutexGuard<'_, Inner> {
    self.0.lock().expect("persistent-window registry poisoned")
  }
}

use crate::hands::{FingerState, HandsState};

/// Describes metric used to measure keyboard layout efficiency.
pub trait Metric {
  /// Updates metric's state with data from given `handstate`.
  fn update_once(&mut self, handstate: &HandsState);
  
  /// Updates metric's state with data from given `handstates`.
  fn update(&mut self, handstates: &[HandsState]) {
    for hs in handstates {
      self.update_once(hs);
    }
  }

  /// Returns metric's score. The lower - the better.
  fn score(&self) -> f32;
}

/// Measures finger usage.
#[derive(Debug, Default)]
pub struct FingerUsage([u32; 10]);

impl FingerUsage {
  pub fn new() -> Self {
    Self([0; 10])
  }
}

impl Metric for FingerUsage {
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, fs) in self.0.iter_mut().zip(handstate.iter()) {
      *fc += *fs as u32;
    }
  }

  fn score(&self) -> f32 {
    self.0.map(|v| v as f32).iter().sum()
  }
}

/// Measures hand usage.
#[derive(Debug, Default)]
pub struct HandUsage([u32; 2]);

impl HandUsage {
  pub fn new() -> Self {
    Self([0; 2])
  }
}

impl Metric for HandUsage {
  fn update_once(&mut self, handstate: &HandsState) {
    for (hc, hs) in self.0.iter_mut().zip(handstate.chunks(2)) {
      *hc += hs.iter().map(|fs| *fs as u32).sum::<u32>();
    }
  }

  fn score(&self) -> f32 {
    self.0.map(|v| v as f32).iter().sum()
  }
}

impl From<FingerUsage> for HandUsage {
  fn from(value: FingerUsage) -> Self {
    let (lh, rh) = value.0.split_at(5);
    Self([lh.iter().sum(), rh.iter().sum()])
  }
}

/// Measures finger alternation.
#[derive(Debug, Default)]
pub struct FingerAlternation {
  last_handstate: HandsState,
  consecutive_presses: [u32; 10],
}

impl FingerAlternation {
  pub fn new() -> Self {
    Self {
      last_handstate: [0; 10].into(),
      consecutive_presses: [0; 10],
    }
  }
}

impl Metric for FingerAlternation {
  fn update_once(&mut self, handstate: &HandsState) {
    for (cp, (last_fs, curr_fs)) in self
      .consecutive_presses
      .iter_mut()
      .zip(self.last_handstate.iter().zip(handstate.iter()))
    {
      if *last_fs == FingerState::Pressed && *curr_fs == FingerState::Pressed {
        *cp += 1;
      }
    }
    self.last_handstate = *handstate;
  }

  fn score(&self) -> f32 {
    self.consecutive_presses.map(|v| v as f32).iter().sum()
  }
}

/// Measures hand alternation.
#[derive(Debug, Default)]
pub struct HandAlternation {
  last_hands_used: [bool; 2],
  consecutive_presses: [u32; 2],
}

impl HandAlternation {
  pub fn new() -> Self {
    Self {
      last_hands_used: [false; 2],
      consecutive_presses: [0; 2],
    }
  }
}

impl Metric for HandAlternation {
  fn update_once(&mut self, handstate: &HandsState) {
    for (cp, (last_hand_used, curr_hs)) in self
      .consecutive_presses
      .iter_mut()
      .zip(self.last_hands_used.iter_mut().zip(handstate.chunks(2)))
    {
      let next_hand_used = curr_hs.iter().any(|fs| *fs == FingerState::Pressed);
      if *last_hand_used && next_hand_used {
        *cp += 1;
      }
      *last_hand_used = next_hand_used;
    }
  }

  fn score(&self) -> f32 {
    self.consecutive_presses.map(|v| v as f32).iter().sum()
  }
}

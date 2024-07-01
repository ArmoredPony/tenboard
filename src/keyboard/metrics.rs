use crate::hands::{FingerState, HandsState};

/// Describes metric used to measure keyboard layout efficiency.
pub trait Metric: Sized {
  /// Updates metric's state with data from given `handstate`.
  fn update_once(&mut self, handstate: &HandsState);
  
  /// Updates metric's state with data from given `handstates`.
  fn update(&mut self, handstates: &[HandsState]) {
    for hs in handstates {
      self.update_once(hs);
    }
  }

  /// Consumes `self`, then `update`s and returns it.
  fn updated(mut self, handstates: &[HandsState]) -> Self {
    self.update(handstates);
    self
  }

  /// Returns metric's score. The lower - the better.
  fn score(&self) -> f32;
}

/// Measures finger usage.
#[derive(Debug, Default)]
pub struct FingerUsage {
  presses: [u32; 10]
}

impl FingerUsage {
  pub fn new() -> Self {
    Self {
      presses: [0; 10]
    }
  }
}

impl Metric for FingerUsage {
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, fs) in self.presses.iter_mut().zip(handstate.iter()) {
      *fc += *fs as u32;
    }
  }

  fn score(&self) -> f32 {
    self.presses.map(|v| v as f32).iter().sum()
  }
}

/// Measures hand usage.
#[derive(Debug, Default)]
pub struct HandUsage {
  presses: [u32; 2]
}

impl HandUsage {
  pub fn new() -> Self {
    Self {
      presses: [0; 2]
    }
  }
}

impl Metric for HandUsage {
  fn update_once(&mut self, handstate: &HandsState) {
    for (hc, hs) in self.presses.iter_mut().zip(handstate.chunks(2)) {
      *hc += hs.iter().map(|fs| *fs as u32).sum::<u32>();
    }
  }

  fn score(&self) -> f32 {
    self.presses.map(|v| v as f32).iter().sum()
  }
}

impl From<FingerUsage> for HandUsage {
  fn from(value: FingerUsage) -> Self {
    let (lh, rh) = value.presses.split_at(5);
    Self {
      presses: [lh.iter().sum(), rh.iter().sum()]
    }
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

/// Measures finger usage balance. Compares it to target balance ratio.
#[derive(Debug, Default)]
pub struct FingerBalance{
  presses: [u32; 10],
  target_ratio: [f32; 10]
}

impl FingerBalance {
  pub fn set_ratio(&mut self, target_ratio: [f32; 10]) -> &mut Self {
    let sum = target_ratio.iter().sum::<f32>();
    self.target_ratio = target_ratio.map(|r| r / sum);
    self
  }
  
  pub fn new() -> Self {
    Self {
      presses: [0; 10],
      target_ratio: [0.0; 10]
    }
  }
  
  pub fn new_with_ratio(target_ratio: [f32; 10]) -> Self {
    let mut fb = Self::new();
    fb.set_ratio(target_ratio);
    fb
  }
}

impl Metric for FingerBalance{
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, fs) in self.presses.iter_mut().zip(handstate.iter()) {
      *fc += *fs as u32;
    }
  }

  fn score(&self) -> f32 {
    let total_presses = self.presses.iter().sum::<u32>() as f32;
    let ratio = self.presses.map(|v| v as f32 / total_presses);
    ratio.iter().zip(self.target_ratio).map(|(a, b)| (a - b).abs()).sum()
  }
}

impl From<FingerUsage> for FingerBalance {
  fn from(value: FingerUsage) -> Self {
    Self {
      presses: value.presses,
      target_ratio: [0.0; 10]
    }
  }
}

/// Measures hand usage balance. Compares it to target balance ratio.
#[derive(Debug, Default)]
pub struct HandBalance{
  presses: [u32; 2],
  target_ratio: [f32; 2]
}

impl HandBalance {
  pub fn set_ratio(&mut self, target_ratio: [f32; 2]) -> &mut Self {
    let sum = target_ratio.iter().sum::<f32>();
    self.target_ratio = target_ratio.map(|r| r / sum);
    self
  }
  
  pub fn new() -> Self {
    Self {
      presses: [0; 2],
      target_ratio: [0.0; 2]
    }
  }
  
  pub fn new_with_ratio(target_ratio: [f32; 2]) -> Self {
    let mut fb = Self::new();
    fb.set_ratio(target_ratio);
    fb
  }
}

impl Metric for HandBalance{
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, fs) in self.presses.iter_mut().zip(handstate.iter()) {
      *fc += *fs as u32;
    }
  }

  fn score(&self) -> f32 {
    let total_presses = self.presses.iter().sum::<u32>() as f32;
    let ratio = self.presses.map(|v| v as f32 / total_presses);
    ratio.iter().zip(self.target_ratio).map(|(a, b)| (a - b).abs()).sum()
  }
}

impl From<HandUsage> for HandBalance {
  fn from(value: HandUsage) -> Self {
    Self {
      presses: value.presses,
      target_ratio: [0.0; 2]
    }
  }
}

impl From<FingerBalance> for HandBalance {
  fn from(value: FingerBalance) -> Self {
    Self {
      presses: {
        let (left, right) = value.presses.split_at(5);
        [left.iter().sum(), right.iter().sum()]
      },
      target_ratio: {
        let (left, right) = value.target_ratio.split_at(5);
        [left.iter().sum(), right.iter().sum()]
      },
    }
  }
}
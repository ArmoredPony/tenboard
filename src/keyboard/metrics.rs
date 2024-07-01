use crate::hands::{FingerState, HandsState};

pub trait Metric {
  /// Updates metric's state with data from given `handstate`.
  fn update_once(&mut self, handstate: &HandsState);

  fn update(&mut self, handstates: &[HandsState]) {
    for hs in handstates {
      self.update_once(hs);
    }
  }

  /// Returns metric's score. The lower - the better.
  fn score(&self) -> f32;
}

/// Counts presses of each finger.
#[derive(Debug, Default)]
pub struct FingerPresses([u32; 10]);

impl FingerPresses {
  pub fn new() -> Self {
    Self([0; 10])
  }
}

impl Metric for FingerPresses {
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, fs) in self.0.iter_mut().zip(handstate.iter()) {
      *fc += *fs as u32;
    }
  }

  fn score(&self) -> f32 {
    self.0.map(|v| v as f32).iter().sum()
  }
}

/// Counts each hand usage.
#[derive(Debug, Default)]
pub struct HandPresses([u32; 2]);

impl HandPresses {
  pub fn new() -> Self {
    HandPresses([0; 2])
  }
}

impl Metric for HandPresses {
  fn update_once(&mut self, handstate: &HandsState) {
    for (hc, hs) in self.0.iter_mut().zip(handstate.chunks(2)) {
      *hc += hs.iter().map(|fs| *fs as u32).sum::<u32>();
    }
  }

  fn score(&self) -> f32 {
    self.0.map(|v| v as f32).iter().sum()
  }
}

impl From<FingerPresses> for HandPresses {
  fn from(value: FingerPresses) -> Self {
    let (lh, rh) = value.0.split_at(5);
    HandPresses([lh.iter().sum(), rh.iter().sum()])
  }
}

/// Counts consecutive presses of each finger
#[derive(Debug, Default)]
pub struct FingerPressesConsecutive {
  last_handstate: HandsState,
  consecutive_presses: [u32; 10],
}

impl FingerPressesConsecutive {
  pub fn new() -> Self {
    Self {
      last_handstate: [0; 10].into(),
      consecutive_presses: [0; 10],
    }
  }
}

impl Metric for FingerPressesConsecutive {
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

/// Counts consecutive presses of each hand
#[derive(Debug, Default)]
pub struct HandPressesConsecutive {
  last_hands_used: [bool; 2],
  consecutive_presses: [u32; 2],
}

impl HandPressesConsecutive {
  pub fn new() -> Self {
    Self {
      last_hands_used: [false; 2],
      consecutive_presses: [0; 2],
    }
  }
}

impl Metric for HandPressesConsecutive {
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

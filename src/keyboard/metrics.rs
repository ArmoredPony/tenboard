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
  presses: [u32; 10],
}

impl FingerUsage {
  pub fn new() -> Self {
    Self { presses: [0; 10] }
  }
}

impl Metric for FingerUsage {
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, fs) in self.presses.iter_mut().zip(handstate.iter()) {
      *fc += u32::from(*fs);
    }
  }

  fn score(&self) -> f32 {
    self.presses.map(|v| v as f32).iter().sum()
  }
}

/// Measures hand usage.
#[derive(Debug, Default)]
pub struct HandUsage {
  presses: [u32; 2],
}

impl HandUsage {
  pub fn new() -> Self {
    Self { presses: [0; 2] }
  }
}

impl Metric for HandUsage {
  fn update_once(&mut self, handstate: &HandsState) {
    for (hc, hs) in self.presses.iter_mut().zip(handstate.hand_iter()) {
      *hc += hs.iter().map(|fs| u32::from(*fs)).sum::<u32>();
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
      presses: [lh.iter().sum(), rh.iter().sum()],
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
      .zip(self.last_hands_used.iter_mut().zip(handstate.hand_iter()))
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
pub struct FingerBalance {
  presses: [u32; 10],
  target_ratio: [f32; 10],
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
      target_ratio: [0.1; 10],
    }
  }

  pub fn new_with_ratio(target_ratio: [f32; 10]) -> Self {
    let mut fb = Self::new();
    fb.set_ratio(target_ratio);
    fb
  }
}

impl Metric for FingerBalance {
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, fs) in self.presses.iter_mut().zip(handstate.iter()) {
      *fc += u32::from(*fs);
    }
  }

  fn score(&self) -> f32 {
    let total_presses = self.presses.iter().sum::<u32>() as f32;
    let ratio = self.presses.map(|v| v as f32 / total_presses);
    ratio
      .iter()
      .zip(self.target_ratio)
      .map(|(a, b)| (a - b).abs())
      .sum()
  }
}

impl From<FingerUsage> for FingerBalance {
  fn from(value: FingerUsage) -> Self {
    Self {
      presses: value.presses,
      target_ratio: [0.1; 10],
    }
  }
}

/// Measures hand usage balance. Compares it to target balance ratio.
#[derive(Debug, Default)]
pub struct HandBalance {
  presses: [u32; 2],
  target_ratio: [f32; 2],
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
      target_ratio: [0.5; 2],
    }
  }

  pub fn new_with_ratio(target_ratio: [f32; 2]) -> Self {
    let mut fb = Self::new();
    fb.set_ratio(target_ratio);
    fb
  }
}

impl Metric for HandBalance {
  fn update_once(&mut self, handstate: &HandsState) {
    for (fc, hand) in self.presses.iter_mut().zip(handstate.hand_iter()) {
      *fc += hand.iter().map(|fs| u32::from(*fs)).sum::<u32>()
    }
  }

  fn score(&self) -> f32 {
    let total_presses = self.presses.iter().sum::<u32>() as f32;
    let ratio = self.presses.map(|v| v as f32 / total_presses);
    ratio
      .iter()
      .zip(self.target_ratio)
      .map(|(a, b)| (a - b).abs())
      .sum()
  }
}

impl From<HandUsage> for HandBalance {
  fn from(value: HandUsage) -> Self {
    Self {
      presses: value.presses,
      target_ratio: [0.5; 2],
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::keyboard::{Keyboard, NoSuchChar};

  struct TestKeyboard {}

  impl TestKeyboard {
    fn try_type_char(&mut self, ch: char) -> Result<HandsState, NoSuchChar> {
      match ch {
        'a' => Ok([1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()),
        'b' => Ok([0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into()),
        'c' => Ok([0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into()),
        'd' => Ok([0, 0, 0, 0, 0, 0, 0, 1, 0, 0].into()),
        'e' => Ok([0, 0, 0, 0, 0, 0, 0, 0, 1, 0].into()),
        'f' => Ok([0, 0, 0, 0, 0, 0, 0, 0, 0, 1].into()),
        'p' => Ok([0, 0, 0, 1, 0, 0, 0, 0, 0, 0].into()),
        'q' => Ok([0, 0, 0, 0, 1, 0, 0, 0, 0, 0].into()),
        'r' => Ok([0, 0, 0, 0, 0, 1, 0, 0, 0, 0].into()),
        's' => Ok([0, 0, 0, 0, 0, 0, 1, 0, 0, 0].into()),
        _ => Err(NoSuchChar { ch }),
      }
    }
  }

  impl Keyboard for TestKeyboard {
    fn try_type_text(
      &mut self,
      text: &str,
    ) -> Result<Vec<HandsState>, NoSuchChar> {
      text.chars().map(|ch| self.try_type_char(ch)).collect()
    }
  }

  #[test]
  fn test_finger_usage() {
    let mut kb = TestKeyboard {};
    let text = "abcdefadab";
    let fu = FingerUsage::new().updated(&kb.type_text(text));
    assert_eq!(fu.presses, [3, 2, 1, 0, 0, 0, 0, 2, 1, 1]);
    assert_eq!(fu.score(), 10.0);
  }

  #[test]
  fn test_hand_usage() {
    let mut kb = TestKeyboard {};
    let text = "abcdefadab";
    let hu = HandUsage::new().updated(&kb.type_text(text));
    assert_eq!(hu.presses, [6, 4]);
    assert_eq!(hu.score(), 10.0);

    let fu = FingerUsage::new().updated(&kb.type_text(text));
    let hu = HandUsage::from(fu);
    assert_eq!(hu.presses, [6, 4]);
    assert_eq!(hu.score(), 10.0);
  }

  #[test]
  fn test_finger_alternation() {
    let mut kb = TestKeyboard {};
    let text = "abcdef";
    let fa = FingerAlternation::new().updated(&kb.type_text(text));
    assert_eq!(fa.consecutive_presses, [0; 10]);
    assert_eq!(fa.score(), 0.0);

    let text = "aacffeddaaaaba";
    let fa = FingerAlternation::new().updated(&kb.type_text(text));
    assert_eq!(fa.consecutive_presses, [4, 0, 0, 0, 0, 0, 0, 1, 0, 1]);
    assert_eq!(fa.score(), 6.0);
  }

  #[test]
  fn test_hand_alternation() {
    let mut kb = TestKeyboard {};
    let text = "adbecf";
    let ha = HandAlternation::new().updated(&kb.type_text(text));
    assert_eq!(ha.consecutive_presses, [0; 2]);
    assert_eq!(ha.score(), 0.0);

    let text = "abcadefafef";
    let ha = HandAlternation::new().updated(&kb.type_text(text));
    assert_eq!(ha.consecutive_presses, [3, 4]);
    assert_eq!(ha.score(), 7.0);
  }

  #[test]
  fn test_finger_balance() {
    let mut kb = TestKeyboard {};
    let text = "abcdefpqrs";
    let fb = FingerBalance::new().updated(&kb.type_text(text));
    assert_eq!(fb.presses, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(fb.score(), 0.0);

    let fb = FingerBalance::new_with_ratio([
      1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ])
    .updated(&kb.type_text(text));
    assert_eq!(fb.presses, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert!(fb.score() - 1.6 < 1.0e-6);

    let fu = FingerUsage::new().updated(&kb.type_text(text));
    let fb = FingerBalance::from(fu);
    assert_eq!(fb.presses, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(fb.score(), 0.0);
  }

  #[test]
  fn test_hand_balance() {
    let mut kb = TestKeyboard {};
    let text = "abcdefpqrs";
    let hb = HandBalance::new().updated(&kb.type_text(text));
    assert_eq!(hb.presses, [5, 5]);
    assert_eq!(hb.score(), 0.0);

    let hb = HandBalance::new_with_ratio([3.0, 7.0]) //
      .updated(&kb.type_text(text));
    assert_eq!(hb.presses, [5, 5]);
    assert!(hb.score() - 0.4 < 1.0e-6);

    let hu = HandUsage::new().updated(&kb.type_text(text));
    let hb = HandBalance::from(hu);
    assert_eq!(hb.presses, [5, 5]);
    assert_eq!(hb.score(), 0.0);

    let fb = FingerBalance::new().updated(&kb.type_text(text));
    let hb = HandBalance::from(fb);
    assert_eq!(hb.presses, [5, 5]);
    assert_eq!(hb.score(), 0.0);
  }
}

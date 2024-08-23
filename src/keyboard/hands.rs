//! Contains description of hands' and fingers' actions used to type stuff on a
//! keyboard.

use std::{
  fmt::Display,
  ops::{Deref, DerefMut},
  slice::Chunks,
};

/// Represents a finger state. Can be either pressed or released.
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum FingerState {
  Pressed,
  #[default]
  Released,
}

impl FingerState {
  pub fn is_pressed(&self) -> bool {
    *self == Self::Pressed
  }

  pub fn is_released(&self) -> bool {
    *self == Self::Released
  }
}

impl From<bool> for FingerState {
  fn from(value: bool) -> Self {
    match value {
      true => FingerState::Pressed,
      false => FingerState::Released,
    }
  }
}

impl From<i32> for FingerState {
  fn from(value: i32) -> Self {
    FingerState::from(value > 0)
  }
}

impl From<FingerState> for u32 {
  fn from(value: FingerState) -> Self {
    match value {
      FingerState::Pressed => 1,
      FingerState::Released => 0,
    }
  }
}

impl Display for FingerState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FingerState::Pressed => write!(f, "|"),
      FingerState::Released => write!(f, "."),
    }
  }
}

/// Represents state of hands with fingers state with a 10 element long array.
/// That little ASCII art below describes how the fingers are indexed.
/// <pre>
///  0 1 2 3 4  5 6 7 8 9
///    _.-._      _.-._
///  _| | | |    | | | |_
/// | | | | |_  _|       |
/// |        /  \        |
/// </pre>
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub struct HandsState(pub [FingerState; 10]);

impl HandsState {
  /// Returns iterator producing unique `HandsState` objects with one or two
  /// fingers pressed.
  pub fn iterate_unique() -> impl Iterator<Item = HandsState> {
    (0..10).flat_map(|i| {
      (i..10).map(move |j| {
        let mut fs = [0; 10];
        fs[i] = 1;
        fs[j] = 1;
        HandsState::from(fs)
      })
    })
  }

  /// Returns iterator over finger states for left then right hand.
  pub fn hand_iter(&self) -> Chunks<FingerState> {
    self.0.chunks(5)
  }
}

impl From<[i32; 10]> for HandsState {
  fn from(value: [i32; 10]) -> Self {
    HandsState(
      value
        .iter()
        .map(|i| FingerState::from(*i))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap(),
    )
  }
}

impl Deref for HandsState {
  type Target = [FingerState; 10];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for HandsState {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Display for HandsState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (lh, rh) = self.split_at(5);
    lh.iter().try_for_each(|fs| write!(f, "{}", fs))?;
    write!(f, " ")?;
    rh.iter().try_for_each(|fs| write!(f, "{}", fs))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bool_to_finger_state() {
    assert_eq!(FingerState::from(true), FingerState::Pressed);
    assert_eq!(FingerState::from(false), FingerState::Released);
  }

  #[test]
  fn test_int_to_finger_state() {
    assert_eq!(FingerState::from(0), FingerState::Released);
    assert_eq!(FingerState::from(1), FingerState::Pressed);
    assert_eq!(FingerState::from(48492975), FingerState::Pressed);
  }

  #[test]
  fn test_finger_state_to_int() {
    assert_eq!(u32::from(FingerState::Pressed), 1);
    assert_eq!(u32::from(FingerState::Released), 0);
    let x: u32 = 1;
    assert_eq!(x + u32::from(FingerState::Pressed), 2);
  }

  #[test]
  fn test_iterate_inique_handsstates() {
    let handsstates: Vec<_> = HandsState::iterate_unique().collect();
    assert_eq!(handsstates.len(), 55);
    assert!(handsstates.iter().all(|hs| {
      let s = hs.into_iter().filter(FingerState::is_pressed).count();
      s == 1 || s == 2
    }))
  }
}

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
  pub fn left_thumb() -> Self {
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0].into()
  }

  pub fn right_thumb() -> Self {
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0].into()
  }

  /// Returns iterator over unique one key `HandsState`s without left and
  /// right thumbs.
  pub fn iterate_one_key_no_thumbs() -> impl Iterator<Item = HandsState> {
    (0..=4).chain(6..10).map(|i| {
      let mut fs = [0; 10];
      fs[i] = 1;
      fs.into()
    })
  }

  /// Returns iterator over unique two key `HandsState`s without left and
  /// right thumbs.
  pub fn iterate_two_key_no_thumbs() -> impl Iterator<Item = HandsState> {
    (0..7).flat_map(|i| {
      (i..8)
        .filter_map(move |j| {
          if i != j {
            let mut a = [0; 8];
            a[i] = 1;
            a[j] = 1;
            Some(a)
          } else {
            None
          }
        })
        .map(|[a, b, c, d, e, f, g, h]| [a, b, c, d, 0, 0, e, f, g, h].into())
    })
  }

  /// Returns iterator over unique one and two keys `HandsState`s without left
  /// and right thumbs.
  pub fn iterate_one_two_key_no_thumbs() -> impl Iterator<Item = HandsState> {
    Self::iterate_one_key_no_thumbs().chain(Self::iterate_two_key_no_thumbs())
  }

  /// Returns iterator over two key `HandsState`s with and wit left and
  /// right thumbs.
  pub fn iterate_one_two_key_with_thumbs() -> impl Iterator<Item = HandsState> {
    Self::iterate_one_two_keys_no_thumbs()
      .chain(
        Self::iterate_one_two_keys_no_thumbs()
          .map(|hs| hs.combine(&HandsState::left_thumb())),
      )
      .chain(
        Self::iterate_one_two_keys_no_thumbs()
          .map(|hs| hs.combine(&HandsState::right_thumb())),
      )
  }

  /// Returns iterator over finger states for left then right hand.
  pub fn hand_iter(&self) -> Chunks<FingerState> {
    self.0.chunks(5)
  }

  /// Creates a new `HandsState` where fingers from `self` and `other` are in
  /// `Pressed` state.
  pub fn combine(&self, other: &Self) -> Self {
    let mut handstate = self.to_owned();
    handstate.iter_mut().zip(other.iter()).for_each(|(s, o)| {
      if o == &FingerState::Pressed {
        *s = FingerState::Pressed;
      }
    });
    handstate
  }
}

impl From<[i32; 10]> for HandsState {
  fn from(value: [i32; 10]) -> Self {
    HandsState(value.map(|i| FingerState::from(i)).try_into().unwrap())
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
    let handsstates: Vec<_> = HandsState::iterate_two_key().collect();
    assert_eq!(handsstates.len(), 55);
    assert!(handsstates.iter().all(|hs| {
      let s = hs.into_iter().filter(FingerState::is_pressed).count();
      s == 1 || s == 2
    }))
  }

  #[test]
  fn test_handsstate_combine() {
    let left_thumb: HandsState = [0, 0, 0, 0, 1, 0, 0, 0, 0, 0].into();
    let right_thumb: HandsState = [0, 0, 0, 0, 0, 1, 0, 0, 0, 0].into();
    let handstate: HandsState = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0].into();

    assert_eq!(handstate[4], FingerState::Pressed);
    assert_eq!(handstate[5], FingerState::Released);

    let handstate = handstate.combine(&left_thumb);
    assert_eq!(handstate[4], FingerState::Pressed);
    assert_eq!(handstate[5], FingerState::Released);

    let handstate = handstate.combine(&right_thumb);
    assert_eq!(handstate[4], FingerState::Pressed);
    assert_eq!(handstate[5], FingerState::Pressed);
  }
}

//! Contains description of hands' and fingers' actions used to type stuff on a
//! keyboard.

use std::{
  fmt::Display,
  ops::{Deref, DerefMut},
};

/// Represents a finger state. Can be either pressed or released.
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum FingerState {
  Pressed,
  #[default]
  Released,
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

impl From<FingerState> for i32 {
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
      FingerState::Pressed => write!(f, ""),
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
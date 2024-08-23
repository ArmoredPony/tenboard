//! Describes Tenboard keyboard layout.

use std::collections::HashMap;

use crate::keyboard::{hands::HandsState, Keyboard, NoSuchChar};

pub trait Tenboard: Keyboard {
  /// Returns a sequence of hand states that describe necessary finger presses
  /// for given char sequence to be typed. If for some char no combination was
  /// found, this char is silently skipped.
  fn type_chars_skip(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Vec<HandsState>;
}

/// Unconstrained Tenboard layout. Any symbol can be mapped to any combination.
pub struct TenboardUnconstrained {
  layout: HashMap<char, HandsState>,
}

impl TenboardUnconstrained {
  pub fn new(layout: HashMap<char, HandsState>) -> Self {
    Self { layout }
  }
}

impl Keyboard for TenboardUnconstrained {
  fn try_type_chars(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Result<Vec<HandsState>, crate::keyboard::NoSuchChar> {
    chars
      .map(|ch| self.layout.get(&ch).copied().ok_or(NoSuchChar { ch }))
      .collect()
  }
}

impl Tenboard for TenboardUnconstrained {
  fn type_chars_skip(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Vec<HandsState> {
    chars
      .filter_map(|ch| self.layout.get(&ch).copied())
      .collect()
  }
}

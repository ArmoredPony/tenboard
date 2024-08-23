//! Describes Tenboard keyboard layout.

use rand::prelude::*;
use std::collections::HashMap;

use crate::keyboard::{hands::HandsState, Keyboard, NoSuchChar, TYPABLE_SYMBOLS};

pub trait Tenboard: Keyboard {
  /// Creates a new Tenboard keyboard layout where each character
  /// corresponds to a random `HandsState` with one or two keys pressed.
  fn new_random() -> Self;

  /// Returns a hand state that describes necessary finger combination
  /// for given char to be typed. If for some char no combination was found,
  /// returns an error.
  fn try_type_char(&mut self, ch: char) -> Result<HandsState, NoSuchChar>;

  /// Returns a sequence of hand states that describe necessary finger presses
  /// for given char sequence to be typed. If for some char no combination was
  /// found, this char is silently skipped.
  fn type_chars_skip(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Vec<HandsState> {
    chars.filter_map(|ch| self.try_type_char(ch).ok()).collect()
  }
}

impl<T: Tenboard> Keyboard for T {
  fn try_type_chars(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Result<Vec<HandsState>, NoSuchChar> {
    chars.map(|ch| self.try_type_char(ch)).collect()
  }
}

/// Unconstrained Tenboard layout. Any symbol can be mapped to any combination.
pub struct TenboardUnconstrained {
  layout: HashMap<char, HandsState>,
}

impl Tenboard for TenboardUnconstrained {
  fn new_random() -> Self {
    let mut handsstates: Vec<_> = HandsState::iterate_unique().collect();
    handsstates.shuffle(&mut rand::thread_rng());
    Self {
      layout: HashMap::from_iter(TYPABLE_SYMBOLS.chars().zip(handsstates)),
    }
  }

  fn try_type_char(&mut self, ch: char) -> Result<HandsState, NoSuchChar> {
    self.layout.get(&ch).copied().ok_or(NoSuchChar { ch })
  }
}

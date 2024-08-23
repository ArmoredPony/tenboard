//! Describes Tenboard keyboard layout.

use std::collections::HashMap;

use rand::prelude::*;

use crate::keyboard::{
  hands::HandsState, Keyboard, NoSuchChar, DIGIT_CHARS, LOWERCASE_CHARS,
  PUNCTUATION_CHARS, TYPABLE_CHARS,
};

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
    let mut handsstates: Vec<_> =
      HandsState::iterate_one_two_key_all_states().collect();
    handsstates.shuffle(&mut rand::thread_rng());
    Self {
      layout: HashMap::from_iter(TYPABLE_CHARS.chars().zip(handsstates)),
    }
  }

  fn try_type_char(&mut self, ch: char) -> Result<HandsState, NoSuchChar> {
    self.layout.get(&ch).copied().ok_or(NoSuchChar { ch })
  }
}

/// Constrained Tenboard layout.
/// 'whitespace' and 'enter' are bound to single key thumb chords.
pub struct TenboardThumbConstrained {
  layout: HashMap<char, HandsState>,
}

impl Tenboard for TenboardThumbConstrained {
  fn new_random() -> Self {
    let mut handsstates: Vec<_> =
      HandsState::iterate_one_two_key_with_thumbs().collect();
    handsstates.shuffle(&mut rand::thread_rng());
    let chars_iter =
      TYPABLE_CHARS.chars().filter(|&ch| ch != ' ' && ch != '\n');
    Self {
      layout: HashMap::from_iter(chars_iter.zip(handsstates)),
    }
  }

  fn try_type_char(&mut self, ch: char) -> Result<HandsState, NoSuchChar> {
    match ch {
      ' ' => Ok(HandsState::left_thumb()),
      '\n' => Ok(HandsState::right_thumb()),
      _ => self.layout.get(&ch).ok_or(NoSuchChar { ch }).copied(),
    }
  }
}

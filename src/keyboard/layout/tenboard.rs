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

/// Constrained Tenboard layout.
/// 'whitespace' and 'enter' are bound to single key thumb chords,
/// lowercase letters are bound to other 8 single key chords.
/// uppercase characters are bound to lowercase chords + `whitespace_hs`,
/// punctuiation characters and numbers are bound to other chords + `newline_hs`.
pub struct TenboardModifierConstrained {
  whitespace_hs: HandsState,
  newline_hs: HandsState,
  lowercase_digit_layout: HashMap<char, HandsState>,
  punctuation_layout: HashMap<char, HandsState>,
}

impl Tenboard for TenboardModifierConstrained {
  fn new_random() -> Self {
    let mut rng = rand::thread_rng();
    let (whitespace_hs, newline_hs) = if rng.gen_bool(0.5) {
      (HandsState::left_thumb(), HandsState::right_thumb())
    } else {
      (HandsState::right_thumb(), HandsState::left_thumb())
    };
    let mut lowercase_digit_hs: Vec<_> =
      HandsState::iterate_one_two_key_no_thumbs().collect();
    let mut punctuation_hs: Vec<_> =
      HandsState::iterate_one_two_key_no_thumbs()
        .map(|hs| hs.combine(&newline_hs))
        .collect();
    lowercase_digit_hs.shuffle(&mut rng);
    punctuation_hs.shuffle(&mut rng);
    Self {
      whitespace_hs,
      newline_hs,
      lowercase_digit_layout: HashMap::from_iter(
        LOWERCASE_CHARS
          .chars()
          .chain(DIGIT_CHARS.chars())
          .zip(lowercase_digit_hs),
      ),
      punctuation_layout: HashMap::from_iter(
        PUNCTUATION_CHARS
          .chars()
          .filter(|&ch| ch != ' ' && ch != '\n')
          .zip(punctuation_hs),
      ),
    }
  }

  fn try_type_char(&mut self, ch: char) -> Result<HandsState, NoSuchChar> {
    match ch {
      ' ' => Some(self.whitespace_hs),
      '\n' => Some(self.newline_hs),
      _ if ch.is_lowercase() || ch.is_ascii_digit() => {
        self.lowercase_digit_layout.get(&ch).copied()
      }
      _ if ch.is_uppercase() => self
        .lowercase_digit_layout
        .get(&ch.to_ascii_lowercase())
        .map(|hs| hs.combine(&self.whitespace_hs)),
      _ => self.punctuation_layout.get(&ch).copied(),
    }
    .ok_or(NoSuchChar { ch })
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::*;

  #[test]
  fn test_random_type_all_chars() -> Result<(), NoSuchChar> {
    let mut tb = TenboardUnconstrained::new_random();
    let hs_set: HashSet<HandsState> = TYPABLE_CHARS
      .chars()
      .map(|ch| tb.try_type_char(ch))
      .collect::<Result<_, _>>()?;
    assert_eq!(hs_set.len(), TYPABLE_CHARS.len());

    let mut tb = TenboardThumbConstrained::new_random();
    let hs_set: HashSet<HandsState> = TYPABLE_CHARS
      .chars()
      .map(|ch| tb.try_type_char(ch))
      .collect::<Result<_, _>>()?;
    assert_eq!(hs_set.len(), TYPABLE_CHARS.len());

    let mut tb = TenboardModifierConstrained::new_random();
    let hs_set: HashSet<HandsState> = TYPABLE_CHARS
      .chars()
      .map(|ch| tb.try_type_char(ch))
      .collect::<Result<_, _>>()?;
    assert_eq!(hs_set.len(), TYPABLE_CHARS.len());

    Ok(())
  }
}

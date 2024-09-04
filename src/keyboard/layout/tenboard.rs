//! Describes Tenboard keyboard layout.

use std::{
  collections::HashMap,
  fmt::{Debug, Display},
};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::keyboard::{
  hands::HandsState,
  Keyboard,
  NoSuchChar,
  DIGIT_CHARS,
  LOWERCASE_CHARS,
  PUNCTUATION_CHARS,
  TYPABLE_CHARS,
};

pub trait Tenboard {
  /// Creates a new Tenboard keyboard layout where each character
  /// corresponds to a random `HandsState`.
  fn new_random() -> Self
  where
    Self: Sized;

  /// Returns a hand state that describes necessary finger combination
  /// for given char to be typed. If for some char no combination was found,
  /// returns an error.
  fn try_type_char(&self, ch: char) -> Result<HandsState, NoSuchChar>;
}

impl<T: Tenboard> Keyboard for T {
  fn try_type_chars(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Result<Vec<HandsState>, NoSuchChar> {
    chars.map(|ch| self.try_type_char(ch)).collect()
  }
}

impl Debug for dyn Tenboard {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    TYPABLE_CHARS.chars().try_for_each(|ch| {
      let hs = self.try_type_char(ch);
      let ch = match ch {
        '\n' => '⤶',
        '\t' => '⇆',
        ' ' => '⎵',
        _ => ch,
      };
      write!(f, "{ch}\t")?;
      match hs {
        Ok(hs) => write!(f, "{hs}")?,
        Err(_) => write!(f, "no match!")?,
      }
      writeln!(f)
    })
  }
}

impl Display for dyn Tenboard {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self, f) // TODO: reimplement later
  }
}

/// Unconstrained Tenboard layout. Any symbol can be mapped to any combination.
#[derive(Serialize, Deserialize)]
pub struct TenboardUnconstrained {
  #[serde(flatten)]
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

  fn try_type_char(&self, ch: char) -> Result<HandsState, NoSuchChar> {
    self.layout.get(&ch).copied().ok_or(NoSuchChar { ch })
  }
}

/// Constrained Tenboard layout.
/// 'whitespace' and 'enter' are bound to single key thumb chords.
#[derive(Serialize, Deserialize)]
pub struct TenboardThumbConstrained {
  #[serde(rename = " ")]
  whitespace_hs: HandsState,
  #[serde(rename = "\n")]
  newline_hs: HandsState,
  #[serde(flatten)]
  layout: HashMap<char, HandsState>,
}

impl Tenboard for TenboardThumbConstrained {
  fn new_random() -> Self {
    let (whitespace_hs, newline_hs) = if rand::thread_rng().gen_bool(0.5) {
      (HandsState::left_thumb(), HandsState::right_thumb())
    } else {
      (HandsState::right_thumb(), HandsState::left_thumb())
    };
    let mut handsstates: Vec<_> =
      HandsState::iterate_one_two_key_with_thumbs().collect();
    handsstates.shuffle(&mut rand::thread_rng());
    let chars_iter =
      TYPABLE_CHARS.chars().filter(|&ch| ch != ' ' && ch != '\n');
    Self {
      whitespace_hs,
      newline_hs,
      layout: HashMap::from_iter(chars_iter.zip(handsstates)),
    }
  }

  fn try_type_char(&self, ch: char) -> Result<HandsState, NoSuchChar> {
    match ch {
      ' ' => Ok(self.whitespace_hs),
      '\n' => Ok(self.newline_hs),
      _ => self.layout.get(&ch).ok_or(NoSuchChar { ch }).copied(),
    }
  }
}

/// Constrained Tenboard layout.
/// 'whitespace' and 'enter' are bound to single key thumb chords,
/// lowercase letters are bound to other 8 single key chords.
/// uppercase characters are bound to lowercase chords + one of the thumbs,
/// punctuiation characters and numbers are bound to other chords + the other
/// thumb.
#[derive(Serialize, Deserialize)]
pub struct TenboardModifierConstrained {
  #[serde(rename = " ")]
  whitespace_hs: HandsState,
  #[serde(rename = "\n")]
  newline_hs: HandsState,
  #[serde(flatten)]
  lowercase_digit_layout: HashMap<char, HandsState>,
  #[serde(flatten)]
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

  fn try_type_char(&self, ch: char) -> Result<HandsState, NoSuchChar> {
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
  fn test_random_unconstrained_all_chars() {
    let tb = TenboardUnconstrained::new_random();
    let hs_set: HashSet<HandsState> = TYPABLE_CHARS
      .chars()
      .map(|ch| tb.try_type_char(ch))
      .collect::<Result<_, _>>()
      .unwrap();
    assert_eq!(hs_set.len(), TYPABLE_CHARS.len());
    assert!(tb.layout.values().all(|hs| hs.count_pressed() <= 3));
  }

  #[test]
  fn test_random_thumb_constrained_all_chars() {
    let tb = TenboardThumbConstrained::new_random();
    let hs_set: HashSet<HandsState> = TYPABLE_CHARS
      .chars()
      .map(|ch| tb.try_type_char(ch))
      .collect::<Result<_, _>>()
      .unwrap();
    assert_eq!(hs_set.len(), TYPABLE_CHARS.len());
    assert!(tb.layout.values().all(|hs| hs.count_pressed() <= 3));
  }

  #[test]
  fn test_random_modifier_constrained_all_chars() {
    let tb = TenboardModifierConstrained::new_random();
    let hs_set: HashSet<HandsState> = TYPABLE_CHARS
      .chars()
      .map(|ch| tb.try_type_char(ch))
      .collect::<Result<_, _>>()
      .unwrap();
    assert_eq!(hs_set.len(), TYPABLE_CHARS.len());
    assert!(tb
      .lowercase_digit_layout
      .values()
      .all(|hs| hs.count_pressed() <= 2));
    assert!(tb
      .punctuation_layout
      .values()
      .all(|hs| matches!(hs.count_pressed(), 2 | 3)));
  }

  #[test]
  fn test_unconstrained_serialization() -> Result<(), serde_json::Error> {
    let tb = TenboardUnconstrained::new_random();
    let json = serde_json::to_string(&tb)?;
    let tb_de: TenboardUnconstrained = serde_json::from_str(&json)?;
    for k in tb.layout.keys() {
      assert_eq!(tb.layout.get(k), tb_de.layout.get(k))
    }
    Ok(())
  }

  #[test]
  fn test_thumb_constrained_serialization() -> Result<(), serde_json::Error> {
    let tb = TenboardThumbConstrained::new_random();
    let json = serde_json::to_string(&tb)?;
    let tb_de: TenboardThumbConstrained = serde_json::from_str(&json)?;
    for k in tb.layout.keys() {
      assert_eq!(tb.layout.get(k), tb_de.layout.get(k))
    }
    Ok(())
  }

  #[test]
  fn test_modifier_constrained_serialization() -> Result<(), serde_json::Error>
  {
    let tb = TenboardModifierConstrained::new_random();
    let json = serde_json::to_string(&tb)?;
    let tb_de: TenboardModifierConstrained = serde_json::from_str(&json)?;
    assert_eq!(tb.whitespace_hs, tb_de.whitespace_hs);
    assert_eq!(tb.newline_hs, tb_de.newline_hs);
    for k in tb.punctuation_layout.keys() {
      assert_eq!(
        tb.punctuation_layout.get(k),
        tb_de.punctuation_layout.get(k)
      )
    }
    for k in tb.lowercase_digit_layout.keys() {
      assert_eq!(
        tb.lowercase_digit_layout.get(k),
        tb_de.lowercase_digit_layout.get(k)
      )
    }
    Ok(())
  }
}

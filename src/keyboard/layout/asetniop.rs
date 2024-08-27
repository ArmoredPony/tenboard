//! Describes ASETNIOP keyboard layout.

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::keyboard::{
  hands::{FingerState, HandsState},
  Keyboard,
  NoSuchChar,
};

const SWITCH_COMBINATION: HandsState = HandsState([
  FingerState::Pressed,
  FingerState::Released,
  FingerState::Released,
  FingerState::Released,
  FingerState::Released,
  FingerState::Released,
  FingerState::Released,
  FingerState::Released,
  FingerState::Released,
  FingerState::Pressed,
]);

lazy_static! {
static ref LETTERS_LAYOUT: HashMap<char, HandsState> = HashMap::from([
  // lowercase
  ('a', [1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()),
  ('b', [0, 0, 0, 1, 0, 0, 1, 0, 0, 0].into()),
  ('c', [0, 1, 0, 1, 0, 0, 0, 0, 0, 0].into()),
  ('d', [0, 1, 1, 0, 0, 0, 0, 0, 0, 0].into()),
  ('e', [0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into()),
  ('f', [1, 0, 0, 1, 0, 0, 0, 0, 0, 0].into()),
  ('g', [0, 0, 0, 1, 0, 0, 0, 0, 1, 0].into()),
  ('h', [0, 0, 0, 0, 0, 0, 1, 1, 0, 0].into()),
  ('i', [0, 0, 0, 0, 0, 0, 0, 1, 0, 0].into()),
  ('j', [0, 1, 0, 0, 0, 0, 1, 0, 0, 0].into()),
  ('k', [0, 1, 0, 0, 0, 0, 0, 1, 0, 0].into()),
  ('l', [0, 0, 0, 0, 0, 0, 0, 1, 1, 0].into()),
  ('m', [0, 0, 0, 0, 0, 0, 1, 0, 0, 1].into()),
  ('n', [0, 0, 0, 0, 0, 0, 1, 0, 0, 0].into()),
  ('o', [0, 0, 0, 0, 0, 0, 0, 0, 1, 0].into()),
  ('p', [0, 0, 0, 0, 0, 0, 0, 0, 0, 1].into()),
  ('q', [1, 0, 0, 0, 0, 0, 1, 0, 0, 0].into()),
  ('r', [0, 0, 1, 1, 0, 0, 0, 0, 0, 0].into()),
  ('s', [0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into()),
  ('t', [0, 0, 0, 1, 0, 0, 0, 0, 0, 0].into()),
  ('u', [0, 0, 0, 0, 0, 0, 1, 0, 1, 0].into()),
  ('v', [0, 0, 0, 1, 0, 0, 0, 1, 0, 0].into()),
  ('w', [1, 1, 0, 0, 0, 0, 0, 0, 0, 0].into()),
  ('x', [1, 0, 1, 0, 0, 0, 0, 0, 0, 0].into()),
  ('y', [0, 0, 1, 0, 0, 0, 1, 0, 0, 0].into()),
  ('z', [1, 0, 0, 0, 0, 0, 0, 1, 0, 0].into()),
  // uppercase (with shift)
  ('A', [1, 0, 0, 0, 1, 0, 0, 0, 0, 0].into()),
  ('B', [0, 0, 0, 1, 1, 0, 1, 0, 0, 0].into()),
  ('C', [0, 1, 0, 1, 1, 0, 0, 0, 0, 0].into()),
  ('D', [0, 1, 1, 0, 1, 0, 0, 0, 0, 0].into()),
  ('E', [0, 0, 1, 0, 1, 0, 0, 0, 0, 0].into()),
  ('F', [1, 0, 0, 1, 1, 0, 0, 0, 0, 0].into()),
  ('G', [0, 0, 0, 1, 1, 0, 0, 0, 1, 0].into()),
  ('H', [0, 0, 0, 0, 1, 0, 1, 1, 0, 0].into()),
  ('I', [0, 0, 0, 0, 1, 0, 0, 1, 0, 0].into()),
  ('J', [0, 1, 0, 0, 1, 0, 1, 0, 0, 0].into()),
  ('K', [0, 1, 0, 0, 1, 0, 0, 1, 0, 0].into()),
  ('L', [0, 0, 0, 0, 1, 0, 0, 1, 1, 0].into()),
  ('M', [0, 0, 0, 0, 1, 0, 1, 0, 0, 1].into()),
  ('N', [0, 0, 0, 0, 1, 0, 1, 0, 0, 0].into()),
  ('O', [0, 0, 0, 0, 1, 0, 0, 0, 1, 0].into()),
  ('P', [0, 0, 0, 0, 1, 0, 0, 0, 0, 1].into()),
  ('Q', [1, 0, 0, 0, 1, 0, 1, 0, 0, 0].into()),
  ('R', [0, 0, 1, 1, 1, 0, 0, 0, 0, 0].into()),
  ('S', [0, 1, 0, 0, 1, 0, 0, 0, 0, 0].into()),
  ('T', [0, 0, 0, 1, 1, 0, 0, 0, 0, 0].into()),
  ('U', [0, 0, 0, 0, 1, 0, 1, 0, 1, 0].into()),
  ('V', [0, 0, 0, 1, 1, 0, 0, 1, 0, 0].into()),
  ('W', [1, 1, 0, 0, 1, 0, 0, 0, 0, 0].into()),
  ('X', [1, 0, 1, 0, 1, 0, 0, 0, 0, 0].into()),
  ('Y', [0, 0, 1, 0, 1, 0, 1, 0, 0, 0].into()),
  ('Z', [1, 0, 0, 0, 1, 0, 0, 1, 0, 0].into()),
  // symbols (no shift)
  ('!', [0, 0, 0, 0, 0, 0, 0, 1, 0, 1].into()),
  ('\'', [0, 0, 1, 0, 0, 0, 0, 0, 0, 1].into()),
  (';', [0, 0, 0, 0, 0, 0, 0, 0, 1, 1].into()),
  (',', [0, 0, 1, 0, 0, 0, 0, 1, 0, 0].into()),
  ('.', [0, 1, 0, 0, 0, 0, 0, 0, 1, 0].into()),
  ('?', [1, 0, 0, 0, 0, 0, 0, 0, 0, 1].into()),
  ('(', [1, 0, 0, 0, 0, 0, 0, 0, 1, 0].into()),
  (')', [0, 1, 0, 0, 0, 0, 0, 0, 0, 1].into()),
  ('-', [0, 0, 1, 0, 0, 0, 0, 0, 1, 0].into()),
  ('\t', [1, 1, 1, 1, 0, 0, 0, 0, 0, 0].into()),
  ('\n', [0, 0, 0, 0, 0, 0, 1, 1, 1, 1].into()),
  // symbols (with shift)
  ('@', [0, 0, 0, 0, 1, 0, 0, 1, 0, 1].into()),
  ('"', [0, 0, 1, 0, 1, 0, 0, 0, 0, 1].into()),
  (':', [0, 0, 0, 0, 1, 0, 0, 0, 1, 1].into()),
  ('<', [0, 0, 1, 0, 1, 0, 0, 1, 0, 0].into()),
  ('>', [0, 1, 0, 0, 1, 0, 0, 0, 1, 0].into()),
  ('/', [1, 0, 0, 0, 1, 0, 0, 0, 0, 1].into()),
  ('[', [1, 0, 0, 0, 1, 0, 0, 0, 1, 0].into()),
  (']', [0, 1, 0, 0, 1, 0, 0, 0, 0, 1].into()),
  ('_', [0, 0, 1, 0, 1, 0, 0, 0, 1, 0].into()),
]);
static ref SYMBOLS_LAYOUT: HashMap<char, HandsState> = HashMap::from([
  // no shift
  ('1', [1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()),
  ('`', [1, 0, 1, 0, 0, 0, 0, 0, 0, 0].into()),
  ('[', [1, 0, 0, 1, 0, 0, 0, 0, 0, 0].into()),
  ('!', [1, 0, 0, 0, 0, 0, 0, 1, 0, 0].into()),
  ('(', [1, 0, 0, 0, 0, 0, 0, 0, 1, 0].into()),
  ('?', [1, 0, 0, 0, 0, 0, 0, 0, 0, 1].into()),
  ('2', [0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into()),
  ('-', [0, 1, 1, 0, 0, 0, 0, 0, 0, 0].into()),
  ('=', [0, 1, 0, 0, 0, 0, 0, 1, 0, 0].into()),
  ('.', [0, 1, 0, 0, 0, 0, 0, 0, 1, 0].into()),
  (')', [0, 1, 0, 0, 0, 0, 0, 0, 0, 1].into()),
  ('3', [0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into()),
  (',', [0, 0, 1, 0, 0, 0, 0, 0, 1, 0].into()),
  ('\'', [0, 0, 1, 0, 0, 0, 0, 0, 0, 1].into()),
  ('4', [0, 0, 0, 1, 0, 0, 0, 0, 0, 0].into()),
  ('5', [0, 0, 1, 1, 0, 0, 0, 0, 0, 0].into()),
  ('6', [0, 0, 0, 0, 0, 0, 1, 1, 0, 0].into()),
  ('7', [0, 0, 0, 0, 0, 0, 1, 0, 0, 0].into()),
  (']', [0, 0, 0, 0, 0, 0, 1, 0, 0, 1].into()),
  ('8', [0, 0, 0, 0, 0, 0, 0, 1, 0, 0].into()),
  ('9', [0, 0, 0, 0, 0, 0, 0, 0, 1, 0].into()),
  (';', [0, 0, 0, 0, 0, 0, 0, 0, 1, 1].into()),
  // with shift
  ('~', [1, 0, 1, 0, 1, 0, 0, 0, 0, 0].into()),
  ('{', [1, 0, 0, 1, 1, 0, 0, 0, 0, 0].into()),
  ('!', [1, 0, 0, 0, 1, 0, 0, 1, 0, 0].into()),
  ('/', [1, 0, 0, 0, 1, 0, 0, 0, 0, 1].into()),
  ('@', [0, 1, 0, 0, 1, 0, 0, 0, 0, 0].into()),
  ('_', [0, 1, 1, 0, 1, 0, 0, 0, 0, 0].into()),
  ('+', [0, 1, 0, 0, 1, 0, 0, 1, 0, 0].into()),
  ('>', [0, 1, 0, 0, 1, 0, 0, 0, 1, 0].into()),
  ('#', [0, 0, 1, 0, 1, 0, 0, 0, 0, 0].into()),
  ('%', [0, 0, 1, 1, 1, 0, 0, 0, 0, 0].into()),
  ('<', [0, 0, 1, 0, 1, 0, 0, 1, 0, 0].into()),
  ('$', [0, 0, 0, 1, 1, 0, 0, 0, 0, 0].into()),
  ('&', [0, 0, 0, 0, 1, 0, 1, 0, 0, 0].into()),
  ('^', [0, 0, 0, 0, 1, 0, 1, 1, 0, 0].into()),
  ('}', [0, 0, 0, 0, 1, 0, 1, 0, 0, 1].into()),
  ('*', [0, 0, 0, 0, 1, 0, 0, 1, 0, 0].into()),
  (':', [0, 0, 0, 0, 1, 0, 0, 0, 1, 1].into()),
]);
}

enum Layout {
  Letters(&'static HashMap<char, HandsState>),
  Symbols(&'static HashMap<char, HandsState>),
}

impl Layout {
  fn new_letters() -> Layout {
    Layout::Letters(&LETTERS_LAYOUT)
  }

  fn new_symbols() -> Layout {
    Layout::Symbols(&SYMBOLS_LAYOUT)
  }

  /// Swaps the layout from letters to symbols.
  fn swap(&mut self) {
    match self {
      Layout::Letters(_) => *self = Self::new_symbols(),
      Layout::Symbols(_) => *self = Self::new_letters(),
    }
  }
}

impl Default for Layout {
  fn default() -> Self {
    Self::new_letters()
  }
}

#[derive(Default)]
pub struct Asetniop {
  layout: Layout,
}

impl Keyboard for Asetniop {
  fn try_type_chars(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Result<Vec<HandsState>, NoSuchChar> {
    let mut handstates: Vec<HandsState> = Vec::new();
    for ch in chars {
      let maybe_hs = match self.layout {
        Layout::Letters(l) => l.get(&ch),
        Layout::Symbols(l) => l.get(&ch),
      };
      if let Some(hs) = maybe_hs {
        handstates.push(hs.to_owned());
      }
      self.layout.swap();
      let maybe_hs = match self.layout {
        Layout::Letters(l) => l.get(&ch),
        Layout::Symbols(l) => l.get(&ch),
      };
      if let Some(hs) = maybe_hs {
        handstates.push(SWITCH_COMBINATION.to_owned());
        handstates.push(hs.to_owned());
      } else {
        return Err(NoSuchChar { ch });
      }
    }
    Ok(handstates)
  }
}

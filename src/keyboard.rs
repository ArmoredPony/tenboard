pub mod metrics;

use std::fmt::Display;

use crate::hands::HandsState;

/// Represents a generic keyboard.
pub trait Keyboard {
  /// Returns a sequence of hand states that describe necessary finger presses
  /// for that text to be typed or an error if that char can't be typed with
  /// this keyboard.
  fn try_type_text(
    &mut self,
    text: &str,
  ) -> Result<Vec<HandsState>, NoSuchChar>;

  /// Emulates typing a text with such keyboard.
  /// Returns a sequence of hand states that describe necessary finger presses
  /// for that text to be typed.
  ///
  /// # Panics
  ///
  /// Panics if any char in the text cannot be typed with this keyboard.
  /// To avoid panic, use [Keyboard::try_type_text].
  fn type_text(&mut self, text: &str) -> Vec<HandsState> {
    self.try_type_text(text).unwrap_or_else(|e| panic!("{e}"))
  }
}

/// This error means that a character couldn't be typed with a `Keyboard`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NoSuchChar {
  pub ch: char,
}

impl Display for NoSuchChar {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "char {} was not found in keyboard", self.ch)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::hands::HandsState;

  struct TestKeyboard {}

  impl Keyboard for TestKeyboard {
    fn try_type_text(
      &mut self,
      text: &str,
    ) -> Result<Vec<HandsState>, NoSuchChar> {
      text
        .chars()
        .map(|ch| match ch {
          'a' => Ok([1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()),
          'b' => Ok([0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into()),
          'c' => Ok([0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into()),
          _ => Err(NoSuchChar { ch }),
        })
        .collect()
    }
  }

  #[test]
  fn test_typing() {
    let mut tk = TestKeyboard {};
    let text = "cabcab";
    assert_eq!(tk.type_text(text), vec![
      [0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into(),
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into(),
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into(),
      [0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into(),
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into(),
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into(),
    ]);
  }

  #[test]
  fn test_char_not_found() {
    let mut tk = TestKeyboard {};
    let text = "abcX";
    assert_eq!(tk.try_type_text(text), Err(NoSuchChar { ch: 'X' }));
  }

  #[test]
  #[should_panic(expected = "char X was not found in keyboard")]
  fn test_char_not_found_panic() {
    let mut tk = TestKeyboard {};
    let text = "abcX";
    tk.type_text(text);
  }
}

pub mod hands;
pub mod layout;
pub mod metrics;

use std::fmt::Display;

use hands::HandsState;

pub const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
pub const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const DIGIT_CHARS: &str = "01234567890";
pub const PUNCTUATION_CHARS: &str = "`-=[]\\;',./~!@#$%^&*()_+{}|:\"<>? \t\n";
pub const TYPABLE_CHARS: &str = concat!(
  "abcdefghijklmnopqrstuvwxyz",
  "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
  "`1234567890-=[]\\;',./",
  "~!@#$%^&*()_+{}|:\"<>?",
  " \t\n"
);

/// Represents a generic keyboard.
pub trait Keyboard {
  /// Returns a sequence of hand states that describe necessary finger presses
  /// for given char sequence to be typed or an error if a char can't be
  /// typed with this keyboard.
  fn try_type_chars(
    &mut self,
    chars: impl Iterator<Item = char>,
  ) -> Result<Vec<HandsState>, NoSuchChar>;

  /// Returns a sequence of hand states that describe necessary finger presses
  /// for given char sequence to be typed.
  ///
  /// # Panics
  ///
  /// Panics if any char in the sequence cannot be typed with this keyboard.
  /// To avoid panic, use [Keyboard::try_type_chars].
  fn type_chars(
    &mut self,
    text: impl Iterator<Item = char>,
  ) -> Vec<HandsState> {
    self.try_type_chars(text).unwrap_or_else(|e| panic!("{e}"))
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

  struct TestKeyboard {}

  impl TestKeyboard {
    fn try_type_char(&mut self, ch: char) -> Result<HandsState, NoSuchChar> {
      match ch {
        'a' => Ok([1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()),
        'b' => Ok([0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into()),
        'c' => Ok([0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into()),
        _ => Err(NoSuchChar { ch }),
      }
    }
  }

  impl Keyboard for TestKeyboard {
    fn try_type_chars(
      &mut self,
      chars: impl Iterator<Item = char>,
    ) -> Result<Vec<HandsState>, NoSuchChar> {
      chars.map(|ch| self.try_type_char(ch)).collect()
    }
  }

  #[test]
  fn test_typing() {
    let mut tk = TestKeyboard {};
    let text = "cabcab";
    assert_eq!(
      tk.type_chars(text.chars()),
      vec![
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into(),
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into(),
        [0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into(),
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0].into(),
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0].into(),
        [0, 1, 0, 0, 0, 0, 0, 0, 0, 0].into(),
      ]
    );
  }

  #[test]
  fn test_char_not_found() {
    let mut tk = TestKeyboard {};
    let text = "abcX";
    assert_eq!(tk.try_type_chars(text.chars()), Err(NoSuchChar { ch: 'X' }));
  }

  #[test]
  #[should_panic(expected = "char X was not found in keyboard")]
  fn test_char_not_found_panic() {
    let mut tk = TestKeyboard {};
    let text = "abcX";
    tk.type_chars(text.chars());
  }
}

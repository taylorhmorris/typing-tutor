use console::{style, Term};
use std::io::{self, Write};

pub struct Phrase {
	pub content: String,
	pub index: usize,
}

impl Phrase {
	pub fn handle_input(&mut self, character: char) -> bool {
		if self.index >= self.content.len() {
			return true;
		}
		if character == self.content.chars().nth(self.index).unwrap() {
			print!("{}", style(character).green());
			io::stdout().flush().unwrap();
			self.index += 1;
		} else {
			print!("{}", style(character).red());
			io::stdout().flush().unwrap();
			let stdout = Term::stdout();
			stdout.move_cursor_left(1).unwrap();
		}
		if self.index >= self.content.len() {
			return true;
		}
		return false;
	}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_returns_true_after_end() {
    let mut phrase = Phrase {
      content: "test".to_string(),
      index: 5,
    };
    let result = phrase.handle_input('t');
    assert_eq!(result, true);
  }

  #[test]
  fn test_returns_false_before_end() {
    let mut phrase = Phrase {
      content: "test".to_string(),
      index: 2,
    };
    let result = phrase.handle_input('t');
    assert_eq!(result, false);
  }

  #[test]
  fn test_advance_on_correct() {
    let mut phrase = Phrase {
      content: "test".to_string(),
      index: 0,
    };
    let result = phrase.handle_input('t');
    assert_eq!(result, false);
    assert_eq!(phrase.index, 1);
  }

  #[test]
  fn test_no_advance_on_incorrect() {
    let mut phrase = Phrase {
      content: "test".to_string(),
      index: 0,
    };
    let result = phrase.handle_input('s');
    assert_eq!(result, false);
    assert_eq!(phrase.index, 0);
  }

  #[test]
  fn test_advance_and_true_on_correct_at_end() {
    let mut phrase = Phrase {
      content: "test".to_string(),
      index: 3,
    };
    let result = phrase.handle_input('t');
    assert_eq!(result, true);
    assert_eq!(phrase.index, 4);
  }
}

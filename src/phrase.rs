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

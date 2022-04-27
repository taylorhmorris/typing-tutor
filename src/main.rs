use std::io::{self, Write};
use console::{Term, style};

struct Phrase<'a> {
	content: &'a str,
	index: usize,
}

impl Phrase<'_> {
	fn handle_input(&mut self, character: char) -> bool {
		if self.index >= self.content.len() {
			return true;
		}
		if character == self.content.chars().nth(self.index).unwrap() {
			print!("{}", style(character).green());
			io::stdout().flush().unwrap();
			self.index += 1;
		}
		else {
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

fn main() {
	let stdout = Term::buffered_stdout();
	let mut phrase = Phrase { content: "Hello, World!", index: 0 };

	println!("{}", phrase.content);
	'game_loop: loop {
		if let Ok(character) = stdout.read_char() {
			match character {
				'`' => break 'game_loop,
				_ => {
						if phrase.handle_input(character) {
							println!("You won!");
							break 'game_loop;
						}
					},
			}
		}
	}
}

//! A simple program to help you practice your typing
//!
//! To get started using Cargo
//! ```bash
//! cargo run -- README.md
//! ```
//!
//! You can replace `README.md` with the path to any text file!
//!
//! Then, type the phrases that appear in the prompt.
//! If you press the correct key, it will be printed in green and advance to the next letter.
//! Otherwise, it will print the key in red and wait for you to type the correct key.

use console::{style, Term};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

pub mod phrase;
use phrase::Phrase;

fn play_file<P>(filename: P) -> bool
where
  P: AsRef<Path>,
{
  Term::clear_screen(&Term::stdout()).expect("Failed to clear terminal");
  let stdout = Term::buffered_stdout();
	let mut phrase = Phrase {
		content: "".to_string(),
		index: 0,
	};

  if let Ok(lines) = read_lines(filename) {
		for line in lines {
			phrase.index = 0;
			if let Ok(line_str) = line {
				phrase.content = line_str.clone();
			} else {
				println!("{}", style("Invalid Line Detected! Skipping").yellow());
				continue;
			}
			println!("{}", phrase.content);
			'game_loop: loop {
				if let Ok(character) = stdout.read_char() {
          if phrase.handle_input(character) {
            println!("");
            break 'game_loop;
          }
				}
			}
		}
		return true;
	} else {
		return false;
	}
}

fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => println!("loading {}", args[1]),
    _ => {
      println!("usage: {} <filename>", args[0]);
      println!("\t<filename>: a text file");
      return;
    }
  }
	if play_file(args[1].clone()) {
		println!("\n{}", style("COMPLETED!").green());
	} else {
		println!("Invalid File");
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
	P: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

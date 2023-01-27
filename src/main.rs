use console::{style, Term};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod phrase;
use phrase::Phrase;

fn main() {
	let stdout = Term::buffered_stdout();
	let mut phrase = Phrase {
		content: "".to_string(),
		index: 0,
	};

	if let Ok(lines) = read_lines("./README.md") {
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

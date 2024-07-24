use std::io::Read;
use std::path::PathBuf;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(name = "cat", about = "concatenate files and print on the standard output", version)]
struct Arguments {
	files: Vec<PathBuf>,

	#[arg(short = 'E', long, help = "display $ at end of each line")]
	show_ends: bool,

	#[arg(short = 'n', long, help = "number all output lines")]
	number: bool,

	#[arg(short = 's', long, help = "suppress repeated empty output lines")]
	squeeze_blank: bool,

	#[arg(short = 'T', long, help = "display TAB characters as ^I")]
	show_tabs: bool,
}

fn print_content(content: &str, arguments: &Arguments, printed_lines: &mut usize, last_line_empty: &mut bool) {
	let lines = content.lines();
	for line in lines.clone() {
		let mut line = line.to_string();
		if line == "" {
			if *last_line_empty && arguments.squeeze_blank {
				continue
			}
			*last_line_empty = true;
		} else {
			*last_line_empty = false;
		}
		if arguments.show_ends {
			line.push('$');
		}
		if arguments.show_tabs {
			line = line.replace("\t", "^I");
		}
		if arguments.number {
			println!("{:6}\t{}", printed_lines, line);
		} else {
			println!("{}", line);
		}
		*printed_lines += 1;
	}
}

fn main() {
	let arguments = Arguments::parse();
	let mut printed_lines = 1;
	let mut last_line_empty = false;

	for file in &arguments.files {
		if file.to_string_lossy() == "-" {
			let mut buffer = String::new();
			while let Ok(size) = std::io::stdin().read_line(&mut buffer) {
				print_content(&buffer, &arguments, &mut printed_lines, &mut last_line_empty);
				buffer.clear();
				if size == 0 {
					break;
				}
			}
		} else {
			match std::fs::read_to_string(file.clone()) {
				Ok(content) => print_content(&content, &arguments, &mut printed_lines, &mut last_line_empty),
				Err(err) => eprintln!("cat: {}: {}", file.display(), err),
			}
		}
	}
}

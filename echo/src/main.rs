use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(name = "echo", about = "Prints the given input", version)]
struct Arguments {
	input: Vec<String>,

	#[arg(short = 'n', help = "Do not output a newline")]
	no_newline: bool,

	#[arg(short = 's', help = "Do not separate arguments with spaces")]
	no_separators: bool,

	#[arg(
		short = 'E',
		help = "Disable interpretation of backslash escapes (default)",
		default_value = "true",
		conflicts_with = "enable_backslash_escapes",
	)]
	disable_backslash_escapes: bool,

	#[arg(
		short = 'e',
		help = "Enable interpretation of backslash escapes",
		conflicts_with = "disable_backslash_escapes",
	)]
	enable_backslash_escapes: bool,
}

fn main() {
	let arguments = Arguments::parse();
	let separator = if arguments.no_separators { "" } else { " " };
	let newline = if arguments.no_newline { "" } else { "\n" };
	let input = arguments.input.join(separator) + newline;
	print!("{}", input);
}

mod parser;
mod intepreter;

use std::path::{PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
enum ArgumentError {
	#[error("Could not find file to read")]
	FileNotFound,

	#[error("Could not open file to read")]
	CouldNotOpenFile,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	if !args.path.exists() {
		Err(ArgumentError::FileNotFound)?;
	}

	println!("Parsing file {:#?}", args.path.as_path());

	// we have a tree_entered virtual method u can override, as well as a on_tree_entered signal

	if let Ok(contents) = std::fs::read_to_string(args.path) {
		parser::parse(contents)?;
	} else {
		Err(ArgumentError::CouldNotOpenFile)?;
	}

	Ok(())
}
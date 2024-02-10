#![cfg_attr(debug_assertions, allow(unused))]

// #![no_std]


mod parser;
mod interpreter;

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


	let a: Vec<_> = vec![10];

	if !args.path.exists() {
		Err(ArgumentError::FileNotFound)?;
	}

	println!("Parsing file {:#?}", args.path.as_path());

	if let Ok(contents) = std::fs::read_to_string(args.path) {
		parser::parse(contents)?;
	} else {
		Err(ArgumentError::CouldNotOpenFile)?;
	}

	Ok(())
}
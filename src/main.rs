#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use clap::{App, Arg, SubCommand, Values};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;

#[derive(Debug)]
struct PasteFile {
	name: String,
	contents: String,
}

impl PasteFile {
	pub fn new(name: &str) -> Result<Self, String> {
		let file = match File::open(name) {
			Ok(file) => file,
			Err(e) => match e.kind() {
				io::ErrorKind::NotFound => return Err(format!("File not found: {}", name)),
				io::ErrorKind::PermissionDenied => {
					return Err(format!("Permission denied: {}", name))
				}
				_ => return Err(format!("Unknown error: {}", name)),
			},
		};

		let mut buf_reader = BufReader::new(file);
		let mut contents = String::new();
		buf_reader
			.read_to_string(&mut contents)
			.map_err(|e| e.to_string())?;

		Ok(Self {
			name: name.to_string(),
			contents,
		})
	}
}

fn create_paste(files: &Vec<PasteFile>) {
	println!("{:?}", files);
}

fn main() {
	let app = App::new("yapb")
		.version("0.1.0")
		.about("YAPB CLI utility")
		.author("Matthew Palmer <mspalmer91@gmail.com>")
		.subcommand(
			SubCommand::with_name("new")
				.about("creates a new paste and returns the URL")
				.arg(Arg::with_name("files").required(true).multiple(true)),
		)
		.subcommand(
			SubCommand::with_name("get")
				.about("retreives a paste")
				.arg(Arg::with_name("url").required(true)),
		)
		.get_matches();

	match app.subcommand() {
		("new", Some(matches)) => {
			let file_names: Values<'_> = matches.values_of("files").unwrap();
			let files = file_names
				.map(|name| match PasteFile::new(name) {
					Ok(file) => file,
					Err(e) => {
						eprintln!("{}", e);
						process::exit(1);
					}
				})
				.collect::<Vec<PasteFile>>();
			create_paste(&files);
		}
		("get", Some(matches)) => {
			println!("{:?}", matches);
		}
		_ => {
			eprintln!("{}", app.usage());
			process::exit(1);
		}
	}
}

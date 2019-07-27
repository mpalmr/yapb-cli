#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use clap::{App, Arg, SubCommand};
use std::process;

fn read_files(files: &[&str]) -> Result<(), String> {
	println!("{:?}", files);
	Ok(())
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
			let file_names: Vec<&str> = matches.values_of("files").unwrap().collect();
			if let Err(err) = read_files(&file_names) {
				eprintln!("Error: {}", err);
				process::exit(1);
			}
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

#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod new_paste;

use std::process;
use clap::{App, Arg, SubCommand};
use crate::new_paste::create;

fn run() -> Result<(), String> {
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
			if let Err(e) = create(matches.values_of("files").unwrap()) {
				return Err(e);
			};
			Ok(())
		}
		("get", Some(matches)) => {
			println!("{:?}", matches);
			Ok(())
		}
		_ => Err(app.usage().to_string()),
	}
}

fn main() {
	if let Err(e) = run() {
		eprintln!("Error: {}", e);
		process::exit(1);
	}
}

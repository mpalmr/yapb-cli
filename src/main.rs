#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod new_paste;

use crate::new_paste::create;
use clap::{App, Arg, SubCommand};
use std::process;

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
			if let Err(e) = create(matches.values_of("files").unwrap()) {
				eprintln!("Error: {}", e);
				process::exit(1);
			};
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

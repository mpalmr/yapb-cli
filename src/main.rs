#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod new_paste;

use crate::new_paste::create;
use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
	let app = App::new("yapb")
		.version("0.1.0")
		.about("Yet Another Paste Bin CLI utility")
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
		("new", Some(subcmd)) => {
			create(subcmd.values_of("files").unwrap())?;
			Ok(())
		}
		("get", Some(subcmd)) => {
			println!("{:?}", subcmd);
			Ok(())
		}
		_ => Err(Box::from(app.usage())),
	}
}

fn main() {
	if let Err(e) = run() {
		eprintln!("{}", e);
		process::exit(1);
	}
}

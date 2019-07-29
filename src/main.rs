#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod get_paste;
mod new_paste;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::process;

#[cfg(debug_assertions)]
const HTTP_ORIGIN: &str = "http://localhost:3000";
#[cfg(not(debug_assertions))]
const HTTP_ORIGIN: &str = "https://yapb.com";

fn run() -> Result<(), Box<dyn Error>> {
	println!("{}", HTTP_ORIGIN);
	let app = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.subcommand(
			SubCommand::with_name("new")
				.about("creates a new paste and returns the URL")
				.arg(Arg::with_name("files").required(true).multiple(true)),
		)
		.subcommand(
			SubCommand::with_name("get")
				.about("retreives a paste")
				.arg(Arg::with_name("id").required(true))
				.arg(Arg::with_name("target").default_value(".")),
		)
		.get_matches();

	match app.subcommand() {
		("new", Some(subcmd)) => {
			new_paste::create(subcmd.values_of("files").unwrap())?;
			Ok(())
		}
		("get", Some(subcmd)) => {
			get_paste::fetch(
				subcmd.value_of("id").unwrap(),
				subcmd.value_of("target").unwrap(),
			)?;
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

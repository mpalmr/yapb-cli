#![warn(
	warnings,
	rust_2018_idioms,
	clippy::all,
	clippy::complexity,
	clippy::correctness,
	clippy::pedantic,
	clippy::perf,
	clippy::style
)]

mod auth;
mod paste;
mod subcommands;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::process;

#[cfg(debug_assertions)]
const HTTP_ORIGIN: &str = "http://localhost:3000/api";
#[cfg(not(debug_assertions))]
const HTTP_ORIGIN: &str = "http://localhost:3000/api"; // Change to domain when purchased

fn run() -> Result<(), Box<dyn Error>> {
	let app = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.subcommand(
			SubCommand::with_name("get")
				.about("Retreives a paste.")
				.arg(Arg::with_name("id").required(true))
				.arg(Arg::with_name("target")),
		)
		.subcommand(
			SubCommand::with_name("create")
				.about("Creates a new paste and returns the URL.")
				.arg(Arg::with_name("files").required(true).multiple(true)),
		)
		.subcommand(
			SubCommand::with_name("login")
				.about("Authenticates with YAPB's API.")
				.arg(
					Arg::with_name("email")
						.required(true)
						.case_insensitive(true),
				),
		)
		.get_matches();

	match app.subcommand() {
		("get", Some(matches)) => subcommands::get(matches),
		("create", Some(matches)) => subcommands::create(matches),
		("login", Some(matches)) => subcommands::login(matches),
		_ => Err(Box::from(app.usage())),
	}
}

fn main() {
	if let Err(e) = run() {
		eprintln!("{}", e);
		process::exit(1);
	}
}

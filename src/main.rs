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

mod get_paste;
mod login;
mod new_paste;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::process;

#[cfg(debug_assertions)]
const HTTP_ORIGIN: &str = "http://localhost:3000";
#[cfg(not(debug_assertions))]
const HTTP_ORIGIN: &str = "http://localhost:3000"; // Change to domain when purchased

fn run() -> Result<(), Box<dyn Error>> {
	let app = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.subcommand(
			SubCommand::with_name("new")
				.about("Creates a new paste and returns the URL.")
				.arg(Arg::with_name("files").required(true).multiple(true)),
		)
		.subcommand(
			SubCommand::with_name("get")
				.about("Retreives a paste.")
				.arg(Arg::with_name("id").required(true))
				.arg(Arg::with_name("target")),
		)
		.subcommand(
			SubCommand::with_name("login")
				.about("Authenticates with YAPB's API.")
				.arg(Arg::with_name("email").required(true))
				.arg(Arg::with_name("password").required(true)),
		)
		.get_matches();

	match app.subcommand() {
		("new", Some(subcmd)) => new_paste::create(subcmd.values_of("files").unwrap()),
		("get", Some(subcmd)) => {
			get_paste::fetch(subcmd.value_of("id").unwrap(), subcmd.value_of("target"))
		}
		("login", Some(subcmd)) => login::login(
			subcmd.value_of("email").unwrap(),
			subcmd.value_of("password").unwrap(),
		),
		_ => Err(Box::from(app.usage())),
	}
}

fn main() {
	if let Err(e) = run() {
		eprintln!("{}", e);
		process::exit(1);
	}
}

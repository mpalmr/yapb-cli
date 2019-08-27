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

use clap::{App, Arg, ArgMatches, SubCommand};
use std::error::Error;
use std::process;

#[cfg(debug_assertions)]
const HTTP_ORIGIN: &str = "http://localhost:3000/api";
#[cfg(not(debug_assertions))]
const HTTP_ORIGIN: &str = "http://localhost:3000/api"; // Change to domain when purchased

pub struct Cli {
	verbose: bool,
}

impl Cli {
	pub fn new(app: &ArgMatches<'_>) -> Self {
		Self {
			verbose: app.is_present("verbose"),
		}
	}

	pub fn log(&self, message: &str) {
		if self.verbose {
			println!("{}", message);
		}
	}
}

fn run() -> Result<(), Box<dyn Error>> {
	let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.arg(Arg::with_name("verbose").short("v"))
		.subcommand(
			SubCommand::with_name("get")
				.about("Retreives a paste")
				.arg(Arg::with_name("id").required(true).help("Paste ID"))
				.arg(Arg::with_name("target").help("Location to download paste to")),
		)
		.subcommand(
			SubCommand::with_name("create")
				.about("Creates a new paste and returns the URL")
				.arg(
					Arg::with_name("files")
						.required(true)
						.multiple(true)
						.help("Glob patterns for matching files to be included in paste"),
				),
		)
		.subcommand(
			SubCommand::with_name("login")
				.about("Authenticates with YAPB's API")
				.arg(
					Arg::with_name("email")
						.required(true)
						.case_insensitive(true)
						.help("Email associated with your YAPB account"),
				),
		)
		.get_matches();

	let cli = Cli::new(&matches);
	match matches.subcommand() {
		("get", Some(subcmd)) => subcommands::get(cli, subcmd),
		("create", Some(subcmd)) => subcommands::create(cli, subcmd),
		("login", Some(subcmd)) => subcommands::login(cli, subcmd),
		_ => Err(Box::from(matches.usage())),
	}
}

fn main() {
	if let Err(e) = run() {
		eprintln!("{}", e);
		process::exit(1);
	}
}

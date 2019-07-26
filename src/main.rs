#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
	let app = App::new("yapb")
		.version("0.1.0")
		.about("YAPB CLI utility")
		.author("Matthew Palmer <mspalmer91@gmail.com>")
		.subcommand(
			SubCommand::with_name("push")
				.about("Pushes files into a new paste")
				.arg(Arg::with_name("files").multiple(true)),
		)
		.subcommand(SubCommand::with_name("pull").about("Pulls files when given URL"))
		.get_matches();

	match app.subcommand() {
		("push", Some(_cmd_name)) => {
			println!("Push!");
		}
		("pull", Some(_cmd_name)) => {
			println!("Pull!");
		}
		_ => {
			eprintln!("{}", app.usage());
			process::exit(1);
		}
	}
}

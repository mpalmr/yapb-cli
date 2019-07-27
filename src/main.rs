#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use clap::{App, Arg, SubCommand, Values};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
struct CreatePasteRequest {
	files: Vec<PasteFile>,
}

impl CreatePasteRequest {
	pub fn new(files: Vec<PasteFile>) -> Self {
		Self { files }
	}
}

#[derive(Deserialize, Debug)]
struct CreatePasteResponse {
	name: String,
}

fn create_paste(files: Vec<PasteFile>) -> Result<CreatePasteResponse, reqwest::Error> {
	let req_payload = CreatePasteRequest::new(files);
	let mut request = match Client::new().post("http://localhost:3000/api/paste").json(&req_payload).send() {
		Ok(request) => request,
		Err(e) => {
			eprintln!("{}", "request_error");
			return Err(e);
		},
	};
	let response = match request.json() {
		Ok(response) => response,
		Err(e) => {
			eprintln!("{}", "response error");
			return Err(e);
		},
	};
	println!("{:#?}", response);
	Ok(response)
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
			let res = match create_paste(files) {
				Ok(res) => res,
				Err(e) => {
					eprintln!("{}", e.to_string());
					process::exit(1);
				}
			};
			println!("EYYYYY");
			println!("{:?}", res);
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

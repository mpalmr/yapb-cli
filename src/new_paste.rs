use clap::Values;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
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

fn create_paste_request(files: Vec<PasteFile>) -> Result<CreatePasteResponse, reqwest::Error> {
	let req_payload = CreatePasteRequest::new(files);
	let mut request = match Client::new()
		.post("http://localhost:3000/api/paste")
		.json(&req_payload)
		.send()
	{
		Ok(request) => request,
		Err(e) => {
			eprintln!("{}", "request_error");
			return Err(e);
		}
	};
	let response = match request.json() {
		Ok(response) => response,
		Err(e) => {
			eprintln!("{}", "response error");
			return Err(e);
		}
	};
	println!("{:#?}", response);
	Ok(response)
}

pub fn create(file_names: Values<'_>) -> Result<(), String> {
	let files = file_names
		.map(|name| match PasteFile::new(name) {
			Ok(file) => file,
			Err(e) => {
				eprintln!("{}", e);
				process::exit(1);
			}
		})
		.collect::<Vec<PasteFile>>();

	let res = match create_paste_request(files) {
		Ok(res) => res,
		Err(e) => {
			eprintln!("{}", e.to_string());
			process::exit(1);
		}
	};

	println!("{:?}", res);
	Ok(())
}

use clap::Values;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

#[derive(Serialize, Debug)]
struct PasteFile {
	name: String,
	contents: String,
}

impl PasteFile {
	pub fn new(name: &str) -> Result<Self, io::Error> {
		let file = File::open(name)?;
		let mut buf_reader = BufReader::new(file);
		let mut contents = String::new();
		buf_reader.read_to_string(&mut contents)?;
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

pub fn create(file_names: Values<'_>) -> Result<(), Box<dyn Error>> {
	match file_names
		.map(|name| PasteFile::new(name))
		.collect::<Result<Vec<PasteFile>, io::Error>>()
	{
		Ok(files) => Ok(Client::new()
			.post("http://localhost:3000/api/paste")
			.json(&CreatePasteRequest::new(files))
			.send()?
			.json()?),
		Err(e) => Err(Box::from(e)),
	}
}

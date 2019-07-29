use clap::Values;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

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
	id: String,
	name: String,
}

pub fn create(file_names: Values<'_>) -> Result<(), Box<dyn Error>> {
	match file_names
		.map(|name| PasteFile::new(name))
		.collect::<Result<Vec<PasteFile>, io::Error>>()
	{
		Ok(files) => {
			let res: CreatePasteResponse = Client::new()
				.post(&format!("{}/api/paste", crate::HTTP_ORIGIN))
				.json(&CreatePasteRequest::new(files))
				.send()?
				.json()?;
			println!("https://localhost:3000/paste/{}", res.id);
			Ok(())
		}
		Err(e) => Err(Box::from(e)),
	}
}

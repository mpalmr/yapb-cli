use crate::HTTP_ORIGIN;
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
			contents,
			name: name.to_string(),
		})
	}
}

#[derive(Deserialize, Debug)]
struct CreatePasteResponse {
	#[serde(rename = "pasteId")]
	paste_id: String,
}

pub fn create(file_names: Values<'_>) -> Result<(), Box<dyn Error>> {
	match file_names
		.map(|name| PasteFile::new(name))
		.collect::<Result<Vec<PasteFile>, io::Error>>()
	{
		Ok(files) => {
			let res: CreatePasteResponse = Client::new()
				.post(&format!("{}/api/paste", HTTP_ORIGIN))
				.json(&files)
				.send()?
				.json()?;
			println!("{}/paste/{}", HTTP_ORIGIN, res.paste_id);
			Ok(())
		}
		Err(e) => Err(Box::from(e)),
	}
}

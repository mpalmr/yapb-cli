use crate::HTTP_ORIGIN;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

#[derive(Deserialize)]
pub struct Paste {
	pub id: String,
	pub files: Vec<File>,
}

impl Paste {
	pub fn fetch(id: &str) -> Result<Self, reqwest::Error> {
		Ok(Client::new()
			.get(&format!("{}/paste/{}", HTTP_ORIGIN, id))
			.send()?
			.json()?)
	}

	pub fn write(&self, out_dir: &Path) -> Result<(), io::Error> {
		create_dir_all(out_dir)?;
		self.files
			.iter()
			.map(|file| file.write(out_dir))
			.collect::<Result<(), io::Error>>()
	}

	pub fn create(files: Vec<File>) -> Result<Self, reqwest::Error> {
		let res: CreatePasteResponse = Client::new()
			.post(&format!("{}/paste", HTTP_ORIGIN))
			.json(&files)
			.send()?
			.json()?;
		Ok(Self {
			id: res.paste_id,
			files,
		})
	}
}

#[derive(Deserialize)]
struct CreatePasteResponse {
	#[serde(rename = "pasteId")]
	paste_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct File {
	pub name: String,
	pub contents: String,
}

impl File {
	pub fn read(name: &str) -> Result<Self, io::Error> {
		let file = fs::File::open(name)?;
		let mut reader = BufReader::new(file);
		let mut contents = String::new();
		reader.read_to_string(&mut contents)?;
		Ok(Self {
			name: name.to_owned(),
			contents,
		})
	}

	pub fn write(&self, out_dir: &Path) -> Result<(), io::Error> {
		fs::File::create(out_dir.join(&self.name))
			.and_then(|mut file| file.write_all(self.contents.as_bytes()))
	}
}

pub fn read_files(file_names: Vec<&str>) -> Result<Vec<File>, io::Error> {
	file_names
		.iter()
		.map(|name| File::read(name))
		.collect::<Result<Vec<File>, io::Error>>()
}

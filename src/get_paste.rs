use crate::HTTP_ORIGIN;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
struct PasteFile {
	name: String,
	contents: String,
}

#[derive(Deserialize)]
struct Paste {
	files: Vec<PasteFile>,
}

pub fn fetch(id: &str, target: Option<&str>) -> Result<(), Box<dyn Error>> {
	let paste: Paste = Client::new()
		.get(&format!("{}/api/paste/{}", HTTP_ORIGIN, id))
		.send()?
		.json()?;
	let target_path = Path::new(target.unwrap_or(id));
	create_dir_all(target_path)?;
	Ok(paste
		.files
		.iter()
		.map(|file| {
			File::create(target_path.join(&file.name))
				.and_then(|mut f| f.write_all(file.contents.as_bytes()))
		})
		.collect::<Result<(), io::Error>>()?)
}

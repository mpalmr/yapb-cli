use crate::HTTP_ORIGIN;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct PasteFile {
	id: String,
	name: String,
	contents: String,
}

#[derive(Deserialize, Debug)]
struct Paste {
	id: String,
	files: Vec<PasteFile>,
}

pub fn fetch(id: &str, target: &str) -> Result<(), Box<dyn Error>> {
	let url = format!("{}/api/paste/{}", HTTP_ORIGIN, id);
	let paste: Paste = Client::new().get(&url).send()?.json()?;
	let path = Path::new(target);
	paste.files.iter().for_each(|file| {
		println!("{:?}", file);
	});
	Ok(())
}

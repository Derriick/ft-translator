use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use encoding_rs_io::DecodeReaderBytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Entry {
	comp_type: String,
	comp_name: String,
	ref_number: u32,
	text: Option<String>,
}

#[derive(Debug, Serialize)]
struct DictEntry {
	src: String,
	dst: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let file = File::open("input.csv")?;
	let mut decoder = DecodeReaderBytes::new(file);
	let mut transcoded = String::new();
	decoder.read_to_string(&mut transcoded)?;

	let mut content = String::new();
	for line in transcoded.lines() {
		if !line.starts_with('#') {
			content.push_str(&line);
			content.push_str("\n");
		}
	}
	
	// Build the CSV reader and iterate over each record.
	let mut rdr = csv::ReaderBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.from_reader(content.as_bytes());

	let mut dict = HashSet::new();
	for result in rdr.deserialize() {
		// The iterator yields Result<StringRecord, Error>, so we check the
		// error here..
		let entry: Entry = result?;
		dict.insert(entry.text);
	}
	let mut dict = dict.into_iter().map(|text| text.unwrap_or(String::new())).collect::<Vec<_>>();
	dict.sort();

	let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').has_headers(false).from_path("output.csv")?;
	for entry in dict {
		let _ = wtr.serialize(DictEntry {src: entry, dst: String::new()})?;
	}
	wtr.flush()?;

	Ok(())
}

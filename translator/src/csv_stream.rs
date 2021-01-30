use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use encoding_rs_io::DecodeReaderBytes;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CsvError {
	#[error("Error in serialization")]
	SerializeError(#[from] csv::Error),
	#[error("Error in parsing")]
	WriteError(#[from] io::Error),
	#[error("Unknown CSV error")]
	Unknown,
}

pub fn read<R>(reader: R) -> csv::Reader<R>
where
	R: io::Read,
{
	csv::ReaderBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.from_reader(reader)
}

pub fn write<I, S, W>(records: I, writer: W) -> Result<(), CsvError>
where
	I: IntoIterator<Item = S>,
	S: Serialize,
	W: io::Write,
{
	let mut writer = csv::WriterBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.from_writer(writer);
	for record in records {
		let _ = writer.serialize(record)?;
	}
	let _ = writer.flush()?;
	Ok(())
}

pub fn read_file<P>(path: P) -> io::Result<String>
where
	P: AsRef<Path>,
{
	let file = File::open(path)?;
	let mut decoder = DecodeReaderBytes::new(file);
	let mut content = String::new();
	decoder.read_to_string(&mut content)?;

	Ok(content
		.lines()
		.skip_while(|l| l.starts_with("# "))
		.collect::<Vec<_>>()
		.join("\n"))
}

pub fn write_file<I, S, P>(records: I, path: P) -> Result<(), CsvError>
where
	I: IntoIterator<Item = S>,
	S: Serialize,
	P: AsRef<Path>,
{
	let writer = File::create(path)?;
	write(records, writer)
}

pub fn write_collec<T>(collection: T, output: Option<&Path>) -> Result<(), CsvError>
where
	T: IntoIterator,
	<T as IntoIterator>::Item: Serialize,
{
	let collection = collection.into_iter().collect::<Vec<_>>();

	match output {
		Some(path) => write_file(collection, path),
		None => write(collection, io::stdout()),
	}
}

pub fn get_vec<T, K, V>(text: &str, get_entry: fn(T) -> (K, V)) -> Result<Vec<(K, V)>, CsvError>
where
	T: for<'de> serde::Deserialize<'de>,
{
	let mut entries = Vec::new();
	for record in read(text.as_bytes()).deserialize() {
		entries.push(get_entry(record?));
	}
	Ok(entries)
}

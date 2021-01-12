use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::io::{self, Read, Write};

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DictError {
	#[error("parsing invalid")]
	ParsingError(#[from] csv::Error),
	#[error("unknown dict error")]
	Unknown,
}

#[derive(Error, Debug)]
pub enum CsvError {
	#[error("invalid serialization")]
	SerializeError(#[from] csv::Error),
	#[error("invalid parsing")]
	WriteError(#[from] io::Error),
	#[error("unknown csv error")]
	Unknown,
}

pub struct Dict(HashMap<String, Option<String>>);

impl IntoIterator for Dict {
	type Item = (String, Option<String>);
	type IntoIter = IntoIter<String, Option<String>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl Dict {
	pub fn from_src(src: &str) -> Result<Dict, DictError> {
		let mut reader = read_csv(src.as_bytes());
		let mut dict = HashMap::new();

		for record in reader.deserialize() {
			let (_, _, _, src): (String, String, String, String) = record?;
			dict.insert(src, None);
		}

		Ok(Dict(dict))
	}

	pub fn from_src_dst(src: &str, dst: &str) -> Result<Dict, DictError> {
		let mut reader_src = read_csv(src.as_bytes());
		let mut reader_dst = read_csv(dst.as_bytes());
		let mut dict = HashMap::new();

		for (record_src, record_dst) in reader_src.deserialize().zip(reader_dst.deserialize()) {
			let (_, _, _, src): (String, String, String, String) = record_src?;
			let (_, _, _, dst): (String, String, String, String) = record_dst?;
			dict.insert(src, Some(dst));
		}

		Ok(Dict(dict))
	}
}

#[inline]
fn read_csv<R>(reader: R) -> csv::Reader<R>
where
	R: Read,
{
	csv::ReaderBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.from_reader(reader)
}

pub fn write_csv<I, S, W>(records: I, writer: W) -> Result<(), CsvError>
where
	I: IntoIterator<Item = S>,
	S: Serialize,
	W: Write,
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

use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::path::Path;

use thiserror::Error;

use crate::csv_stream;

#[derive(Error, Debug)]
pub enum DictError {
	#[error(transparent)]
	WriteError(#[from] io::Error),
	#[error(transparent)]
	CsvStreamError(#[from] csv_stream::CsvError),
	#[error(transparent)]
	CsvError(#[from] csv::Error),
	#[error("unknown dict error")]
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
	pub fn from_src<T>(src: &str, get_text: fn(T) -> String) -> Result<Self, DictError>
	where
		T: for<'de> serde::Deserialize<'de>,
	{
		let mut reader = csv_stream::read(src.as_bytes());
		let mut dict = HashMap::new();

		for record in reader.deserialize() {
			let src = get_text(record?);
			dict.insert(src, None);
		}

		Ok(Dict(dict))
	}

	pub fn from_src_dst<T, K>(
		src: &str,
		dst: &str,
		get_entry: fn(T) -> (K, String),
	) -> Result<Dict, DictError>
	where
		T: for<'de> serde::Deserialize<'de>,
		K: Hash + Eq,
	{
		let mut entries_src = HashMap::new();
		for record in csv_stream::read(src.as_bytes()).deserialize() {
			let (key, value) = get_entry(record?);
			entries_src.insert(key, value);
		}

		let mut entries_dst = HashMap::new();
		for record in csv_stream::read(dst.as_bytes()).deserialize() {
			let (key, value) = get_entry(record?);
			entries_dst.insert(key, value);
		}

		let dict: HashMap<_, _> = entries_src
			.into_iter()
			.map(|(key, value)| (value, entries_dst.get(&key).map(String::from)))
			.collect();
		Ok(Dict(dict))
	}

	pub fn from_file_src<P, T>(path: P, get_text: fn(T) -> String) -> Result<Self, DictError>
	where
		P: AsRef<Path>,
		T: for<'de> serde::Deserialize<'de>,
	{
		let src = csv_stream::read_file(path)?;
		Self::from_src(&src, get_text)
	}

	pub fn from_file_src_dst<P1, P2, T, K>(
		path_src: P1,
		path_dst: P2,
		get_entry: fn(T) -> (K, String),
	) -> Result<Self, DictError>
	where
		P1: AsRef<Path>,
		P2: AsRef<Path>,
		T: for<'de> serde::Deserialize<'de>,
		K: Hash + Eq,
	{
		let src = csv_stream::read_file(path_src)?;
		let dst = csv_stream::read_file(path_dst)?;
		Self::from_src_dst(&src, &dst, get_entry)
	}
}

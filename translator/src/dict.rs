use std::collections::hash_map::{IntoIter, Iter, IterMut};
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

impl Dict {
	pub fn from_dict(dict: &str) -> Result<Self, DictError> {
		let entries = get_entries(dict, |(k, v)| (k, v))?;
		Ok(Dict(entries))
	}

	pub fn from_file_dict<P>(
		path: P,
	) -> Result<Self, DictError>
	where
		P: AsRef<Path>,
	{
		let dict = csv_stream::read_file(path)?;
		Self::from_dict(&dict)
	}

	pub fn from_src<T, K>(src: &str, get_entry: fn(T) -> (K, String)) -> Result<Self, DictError>
	where
		T: for<'de> serde::Deserialize<'de>,
		K: Hash + Eq,
	{
		let mut reader = csv_stream::read(src.as_bytes());
		let mut dict = HashMap::new();
		for record in reader.deserialize() {
			let (_, src) = get_entry(record?);
			dict.insert(src, None);
		}

		Ok(Dict(dict))
	}

	pub fn from_file_src<P, T, K>(
		path: P,
		get_entry: fn(T) -> (K, String),
	) -> Result<Self, DictError>
	where
		P: AsRef<Path>,
		T: for<'de> serde::Deserialize<'de>,
		K: Hash + Eq,
	{
		let src = csv_stream::read_file(path)?;
		Self::from_src(&src, get_entry)
	}

	pub fn from_src_dst<T, K>(
		src: &str,
		dst: &str,
		get_entry: fn(T) -> (K, String),
	) -> Result<Self, DictError>
	where
		T: for<'de> serde::Deserialize<'de>,
		K: Hash + Eq,
	{
		let entries_src = get_entries(src, get_entry)?;
		let entries_dst = get_entries(dst, get_entry)?;

		let dict: HashMap<_, _> = entries_src
			.into_iter()
			.map(|(key, value)| (value, entries_dst.get(&key).map(String::from)))
			.collect();
		Ok(Dict(dict))
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

	pub fn merge(dict1: &str, dict2: &str) -> Result<Self, DictError> {
		let mut entries1 = get_entries(dict1, |(k, v)| (k, v))?;
		let entries2 = get_entries(dict2, |(k, v)| (k, v))?;
		entries1.extend(entries2);
		Ok(Dict(entries1))
	}

	pub fn merge_file<P1, P2>(path_dict1: P1, path_dict2: P2) -> Result<Self, DictError>
	where
		P1: AsRef<Path>,
		P2: AsRef<Path>,
	{
		let dict1 = csv_stream::read_file(path_dict1)?;
		let dict2 = csv_stream::read_file(path_dict2)?;
		Self::merge(&dict1, &dict2)
	}

	pub fn swap(dict: &str) -> Result<Self, DictError> {
		get_entries(dict, |(k, v)| (v, k)).map(Dict)
	}

	pub fn swap_file<P>(path_dict: P) -> Result<Self, DictError>
	where
		P: AsRef<Path>,
	{
		let dict = csv_stream::read_file(path_dict)?;
		Self::swap(&dict)
	}

	/*pub fn translate<T, K>(
		&self,
		src: &str,
		get_entry: fn(T) -> (K, String),
	) -> Result<HashMap<K, String>, DictError>
	where
		T: for<'de> serde::Deserialize<'de>,
		K: Hash + Eq,
	{
		let entries_src = get_entries(src, get_entry)?;

		let entries_dst: HashMap<K, String> = entries_src
			.into_iter()
			.map(|(key, value)| (value, self.0.get(&key).map(String::from).unwrap_or(String::new())))
			.collect();
		Ok(entries_dst)
	}*/
}

impl<'a> IntoIterator for &'a Dict {
	type Item = (&'a String, &'a Option<String>);
	type IntoIter = Iter<'a, String, Option<String>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl<'a> IntoIterator for &'a mut Dict {
	type Item = (&'a String, &'a mut Option<String>);
	type IntoIter = IterMut<'a, String, Option<String>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter_mut()
	}
}

impl IntoIterator for Dict {
	type Item = (String, Option<String>);
	type IntoIter = IntoIter<String, Option<String>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

fn get_entries<T, K, V>(text: &str, get_entry: fn(T) -> (K, V)) -> Result<HashMap<K, V>, DictError>
where
	T: for<'de> serde::Deserialize<'de>,
	K: Hash + Eq,
{
	let mut entries = HashMap::new();
	for record in csv_stream::read(text.as_bytes()).deserialize() {
		let (key, value) = get_entry(record?);
		entries.insert(key, value);
	}
	Ok(entries)
}

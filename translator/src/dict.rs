use std::collections::hash_map::{IntoIter, Iter, IterMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use thiserror::Error;

use crate::csv as csv_helper;

#[derive(Error, Debug)]
pub enum DictError {
	#[error(transparent)]
	WriteError(#[from] io::Error),
	#[error(transparent)]
	CsvStreamError(#[from] csv_helper::CsvError),
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

	pub fn from_src<T, K>(src: &str, get_entry: fn(T) -> (K, String)) -> Result<Self, DictError>
	where
		T: for<'de> serde::Deserialize<'de>,
		K: Hash + Eq,
	{
		let mut reader = csv_helper::read(src.as_bytes());
		let mut dict = HashMap::new();
		for record in reader.deserialize() {
			let (_, src) = get_entry(record?);
			dict.insert(src, None);
		}

		Ok(Dict(dict))
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

	pub fn merge(mut self, dict2: Self) -> Self {
		self.0.extend(dict2.0);
		self
	}

	pub fn swap(self) -> Self {
		let mut dict = HashMap::new();
		for (k, v) in self {
			if let Some(v) = v {
				dict.insert(v, Some(k));
			}
		}
		Dict(dict)
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
	for record in csv_helper::read(text.as_bytes()).deserialize() {
		let (key, value) = get_entry(record?);
		entries.insert(key, value);
	}
	Ok(entries)
}

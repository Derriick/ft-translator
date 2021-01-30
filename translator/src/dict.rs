use std::collections::hash_map::{IntoIter, Iter, IterMut};
use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use serde::Deserialize;
use thiserror::Error;

use crate::csv_stream;
use crate::translation::Translation;

pub struct Dict(HashMap<Translation, Option<Translation>>);

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

impl Dict {
	pub fn from_dict(dict: &str) -> Result<Self, DictError> {
		let mut translations: HashMap<Translation, Option<Translation>> = HashMap::new();
		for (k, v) in csv_stream::get_vec(dict, |(k, v): (String, String)| (k, v))? {
			let key = Translation::from(&k);
			let val = if v.is_empty() {
				None
			} else {
				Some(Translation::from(&v))
			};

			// TODO: handle duplicates: if key already prsent and val different
			translations.insert(key, val);
		}

		Ok(Dict(translations))
	}

	pub fn from_src<T, K>(src: &str, get_entry: fn(T) -> (K, String)) -> Result<Self, DictError>
	where
		T: for<'de> Deserialize<'de>,
		K: Hash + Eq,
	{
		let mut reader = csv_stream::read(src.as_bytes());
		let mut dict = HashMap::new();
		for record in reader.deserialize() {
			let (_, src) = get_entry(record?);
			dict.insert(Translation::from(&src), None);
		}

		Ok(Dict(dict))
	}

	pub fn from_src_dst<T, K>(
		src: &str,
		dst: &str,
		get_entry: fn(T) -> (K, String),
	) -> Result<Self, DictError>
	where
		T: for<'de> Deserialize<'de>,
		K: Hash + Eq,
	{
		let entries_src = csv_stream::get_vec(src, get_entry)?;
		let entries_dst: HashMap<_, _> = csv_stream::get_vec(dst, get_entry)?.into_iter().collect();

		let dict: HashMap<_, _> = entries_src
			.into_iter()
			.map(|(key, value)| {
				(
					Translation::from(&value),
					entries_dst.get(&key).map(|v| Translation::from(&v)),
				)
			})
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

	pub fn translate<T, K>(
		&self,
		src: &str,
		get_entry: fn(T) -> (K, String),
	) -> Result<Vec<(K, String)>, DictError>
	where
		T: for<'de> Deserialize<'de>,
	{
		let entries_src = csv_stream::get_vec(src, get_entry)?;
		let mut entries_dst = Vec::new();
		for (k, v) in entries_src {
			let value = match self.0.get(&Translation::from(&v)) {
				Some(t) => match t {
					Some(t) => t.translate(&v).unwrap_or(v), // TODO: handle errors
					None => String::new(),
				},
				None => String::new(),
			};
			entries_dst.push((k, value));
		}
		Ok(entries_dst)
	}
}

impl<'a> IntoIterator for &'a Dict {
	type Item = (&'a Translation, &'a Option<Translation>);
	type IntoIter = Iter<'a, Translation, Option<Translation>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl<'a> IntoIterator for &'a mut Dict {
	type Item = (&'a Translation, &'a mut Option<Translation>);
	type IntoIter = IterMut<'a, Translation, Option<Translation>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter_mut()
	}
}

impl IntoIterator for Dict {
	type Item = (Translation, Option<Translation>);
	type IntoIter = IntoIter<Translation, Option<Translation>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

use std::collections::HashMap;
use std::collections::hash_map::IntoIter;

use anyhow::Result;

pub struct Dict(HashMap<String, Option<String>>);

impl Dict {
	pub fn from_src(src: &str) -> Result<Dict> {
		let mut rdr = csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.has_headers(false)
			.from_reader(src.as_bytes());

		let mut dict = HashMap::new();
		for record in rdr.deserialize() {
			let (_, _, _, src): (String, String, String, String) = record?;
			dict.insert(src, None);
		}

		Ok(Dict(dict))
	}

	pub fn from_src_dst(src: &str, dst: &str) -> Result<Dict> {
		let mut rdr_src = csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.has_headers(false)
			.from_reader(src.as_bytes());

		let mut rdr_dst = csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.has_headers(false)
			.from_reader(dst.as_bytes());

		let mut dict = HashMap::new();
		for (record_src, record_dst) in rdr_src.deserialize().zip(rdr_dst.deserialize()) {
			let (_, _, _, src): (String, String, String, String) = record_src?;
			let (_, _, _, dst): (String, String, String, String) = record_dst?;
			dict.insert(src, Some(dst));
		}

		Ok(Dict(dict))
	}
}

impl IntoIterator for Dict {
	type Item = (String, Option<String>);
	type IntoIter = IntoIter<String, Option<String>>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}
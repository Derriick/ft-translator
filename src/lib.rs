use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

use anyhow::Result;

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
	pub fn from_src(src: &str) -> Result<Dict> {
		let mut reader = get_reader(src);
		let mut dict = HashMap::new();

		for record in reader.deserialize() {
			let (_, _, _, src): (String, String, String, String) = record?;
			dict.insert(src, None);
		}

		Ok(Dict(dict))
	}

	pub fn from_src_dst(src: &str, dst: &str) -> Result<Dict> {
		let mut reader_src = get_reader(src);
		let mut reader_dst = get_reader(dst);
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
fn get_reader(text: &str) -> csv::Reader<&[u8]> {
	csv::ReaderBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.from_reader(text.as_bytes())
}

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use anyhow::{Context, Result};
use encoding_rs_io::DecodeReaderBytes;
use serde::Serialize;

use ft_translator::*;

pub fn dict_from_src(path: &Path) -> Result<Dict> {
	let src = read_file(path)?;
	Dict::from_src(&src, |record: (String, String, String, String)| record.3).with_context(|| {
		format!(
			"Failed to create dictionary from source file '{}'",
			path.display()
		)
	})
}

pub fn dict_from_src_dst(path_src: &Path, path_dst: &Path) -> Result<Dict> {
	let src = read_file(path_src)?;
	let dst = read_file(path_dst)?;
	Dict::from_src_dst(&src, &dst, |record: (String, String, String, String)| {
		record.3
	})
	.with_context(|| {
		format!(
			"Failed to create dictionary from source file '{}' and dest. file '{}'",
			path_src.display(),
			path_dst.display()
		)
	})
}

pub fn write_csv_file<I, S>(records: I, path: &Path) -> Result<()>
where
	I: IntoIterator<Item = S>,
	S: Serialize,
{
	let writer = File::create(path)?;
	write_csv(records, writer)
		.with_context(|| format!("Failed to write CSV records to file '{}'", path.display()))
}

pub fn write_csv_stdout<I, S>(records: I) -> Result<()>
where
	I: IntoIterator<Item = S>,
	S: Serialize,
{
	write_csv(records, io::stdout())
		.with_context(|| format!("Failed to write CSV records to STDOUT"))
}

fn read_file<P>(path: P) -> Result<String>
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

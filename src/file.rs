use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use encoding_rs_io::DecodeReaderBytes;

use ft_translator::Dict;

#[inline]
pub fn dict_from_src<P>(path: P) -> Result<Dict>
where
	P: AsRef<Path>,
{
	let src = read_csv(path)?;
	Dict::from_src(&src)
}

#[inline]
pub fn dict_from_src_dst<P, Q>(path_src: P, path_dst: Q) -> Result<Dict>
where
	P: AsRef<Path>,
	Q: AsRef<Path>,
{
	let src = read_csv(path_src)?;
	let dst = read_csv(path_dst)?;
	Dict::from_src_dst(&src, &dst)
}

fn read_csv<P>(path: P) -> Result<String>
where
	P: AsRef<Path>,
{
	let file = File::open(path)?;
	let mut decoder = DecodeReaderBytes::new(file);
	let mut content = String::new();
	decoder.read_to_string(&mut content)?;

	Ok(content
		.lines()
		.skip_while(|l| l.starts_with('#'))
		.collect::<Vec<_>>()
		.join("\n"))
}

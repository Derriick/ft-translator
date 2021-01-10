use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, Result};

pub struct Dict(HashMap<String, Option<String>>);

impl Dict {
	pub fn from_src<P: AsRef<Path>>(src: P) -> Result<Dict> {
		todo!();
	}

	pub fn from_src_dst<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<Dict> {
		todo!();
	}
}

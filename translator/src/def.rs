#![allow(dead_code)]
#![allow(unused_variables)]

use serde::ser::{Serialize, SerializeStruct, Serializer};
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Def(Vec<Text>);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Text {
	Translatable(String),
	Key(usize),
}

#[derive(Error, Debug)]
pub enum DefError {
	#[error("unknown def error")]
	Unknown,
}

impl Def {
	pub fn from_text(text: &str) -> Self {
		todo!()
	}
	pub fn from_def(text: &str) -> Self {
		todo!()
	}

	pub fn translate(&self, text: &str) -> Result<String, DefError> {
		// TODO: reconstruct src translation from text to get the keys, apply elf translation
		todo!()
	}
}

impl Serialize for Def {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut s = serializer.serialize_struct("Def", 1)?;
		let text = self
			.0
			.iter()
			.map(|t| match t {
				Text::Translatable(s) => s.clone(),
				Text::Key(k) => format!("{{:{}}}", k),
			})
			.collect::<String>();
		s.serialize_field("text", &text)?;
		s.end()
	}
}

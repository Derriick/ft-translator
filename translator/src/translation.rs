#![allow(dead_code)]
#![allow(unused_variables)]

use serde::ser::{Serialize, SerializeStruct, Serializer};
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Translation(Vec<Text>);

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Text {
	Translatable(String),
	Key(usize),
}

#[derive(Error, Debug)]
pub enum TranslationError {
	#[error("unknown translation error")]
	Unknown,
}

impl Translation {
	pub fn from(text: &str) -> Self {
		todo!()
	}

	pub fn translate(&self, text: &str) -> Result<String, TranslationError> {
		// TODO: reconstruct src translation from text to get the keys, apply elf translation
		todo!()
	}
}

impl Serialize for Translation {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut s = serializer.serialize_struct("Translation", 1)?;
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

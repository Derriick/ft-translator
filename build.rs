use std::env;
use winres;

fn main() {
	if cfg!(target_os = "windows") {
		let mut res = winres::WindowsResource::new();
		res
		//	.set_icon("icon.ico")
			.set("LegalCopyright", env!("CARGO_PKG_AUTHORS"));
		res.compile().unwrap();
	}
}

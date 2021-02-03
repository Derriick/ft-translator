use log::error;

fn main() {
	if let Err(err) =
		translator_cli::run(|(k1, k2, k3, v): (String, String, String, String)| ((k1, k2, k3), v))
	{
		error!("{}", err);
	}
}
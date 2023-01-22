#![allow(unused_variables)]
use wcr::{run, get_args};

fn main() {
	if let Err(err) = get_args().and_then(run) {
		eprintln!("{}", err);
		std::process::exit(1);
	}
}
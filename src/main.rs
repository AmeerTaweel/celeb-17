use structopt::StructOpt;
use url::Url;

mod io;
mod db;
mod log;
mod exit;
mod download;

#[derive(StructOpt)]
#[structopt(
	name = "Celeb-17",
	about = "No Nonsense Command-Line Music Player Written in Rust."
)]
enum Options {
	Download {
		#[structopt(parse(try_from_str))]
		url: Url
	}
}

fn main() {
	let options = Options::from_args();

	match options {
		Options::Download { url } => download::run(url)
	}
}

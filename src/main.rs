use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use inquire::{Text, type_aliases::Suggester};
use url::Url;
use structopt::StructOpt;
use std::process::Command;

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
		Options::Download { url } => download(url)
	}
}

fn prompt_user(prompt: &str, suggestions: &[&str]) -> String {
	let matcher = SkimMatcherV2::default();

    let suggester: Suggester = &|input| {
		let mut suggestions: Vec<(&&str, i64)> = suggestions.iter()
			// Fuzzy match with user input
			.map(|i| (i, matcher.fuzzy_match(i, input).unwrap_or_default()))
			// Remove unmatching suggestions
			.filter(|(i, score)| *score > 0)
			.collect();
		// Sort by score
		suggestions.sort_by(|(_, a), (_, b)| b.cmp(a));
		suggestions.iter().map(|(i, score)| i.to_string()).collect()
	};

	let answer = Text::new(prompt)
		.with_suggester(suggester)
		.prompt();

	match answer {
		Ok(answer) => answer,
		Err(error) => std::process::exit(1)
	}
}

fn download(url: Url) {
	let downloader = Command::new("yt-dlp")
		.arg("--format")
		.arg("bestaudio*")
		.arg("--extract-audio")
		.arg("--audio-format")
		.arg("flac")
		.arg(url.to_string())
		.spawn()
		.expect("Failed to execute yt-dlp.");

	let result = downloader
		.wait_with_output()
		.expect("Failed to wait on yt-dlp.");

	if result.status.success() { println!("Download completed."); }
	else { println!("Download failed."); }
}

use crate::db::{ Database, Song };
use crate::exit::{ self, UnwrapOrExit };
use crate::io;
use crate::log;
use std::process::Command;
use url::Url;

pub fn run(url: Url){
	let mut db = Database::instance();

	let song = Song {
		id: db.get_next_id(),
		name: prompt_name(&db),
		artists: prompt_artists(&db),
		tags: prompt_tags(&db),
		version_of: prompt_version_of(&db),
		url: url.to_string()
	};

	let downloader = Command::new("yt-dlp")
		.arg("--format")
		.arg("bestaudio*")
		.arg("--extract-audio")
		.arg("--audio-format")
		.arg("flac")
		.arg("--paths")
		.arg(&db.music_dir_location)
		.arg("--output")
		.arg(format!("{}.%(ext)s", &song.id))
		.arg(url.to_string())
		.spawn()
		.unwrap_or_exit_with_error("Failed to execute yt-dlp");

	let result = downloader
		.wait_with_output()
		.unwrap_or_exit_with_error("Failed to wait on yt-dlp");

	if result.status.success() {
		db.add_song(song);
		exit::with_success("Download completed");
	} else {
		exit::with_error("Download failed");
	}
}

fn prompt_name(db: &Database) -> String {
	io::prompt_required(
		"Song Name:",
		&db.get_song_names()
	)
}

fn prompt_artists(db: &Database) -> Vec<String> {
	io::prompt_require_n(
		"Artist Name:",
		&db.get_artist_names(),
		1
	)
}

fn prompt_tags(db: &Database) -> Vec<String> {
	io::prompt_require_n(
		"Tag:",
		&db.get_tags(),
		0
	)
}

fn prompt_version_of(db: &Database) -> u64 {
	loop {
		let names = &db.get_song_names();
		let version_of = io::prompt_optional("Version Of:", names);
		if version_of.is_empty() { return 0 }
		if names.contains(&version_of) {
			return db.get_song_by_name(&version_of).id;
		}
		log::info("Song can only be a version of a previously defined song")
	}
}

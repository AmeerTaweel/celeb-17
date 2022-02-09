use crate::exit::{ UnwrapOrExit };
use crate::io;
use serde::{ Deserialize, Serialize };
use std::{ env, fs, path::PathBuf };

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
	pub id: u64,
	pub name: String,
	pub artists: Vec<String>,
	pub tags: Vec<String>,
	pub version_of: u64,
	pub url: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
	pub music_dir_location: String,
	songs: Vec<Song>
}

impl Database {
	fn get_path() -> PathBuf {
		let db_path = env::var("XDG_CONFIG_HOME")
			.expect("Getting database path failed");

		let db_path = format!("{}/celeb-17/db.json", db_path);
		
		PathBuf::from(db_path)
	}

	fn exists() -> bool {
		Database::get_path().is_file()
	}

	fn init() {
		let music_dir_location = io::prompt_user("Where is your music directory located?", &Vec::<String>::new());
		let db = Database { music_dir_location, songs: vec![] };
		let db_path = Database::get_path();
		let db_dir = db_path.parent().unwrap();
		fs::create_dir_all(db_dir)
			.unwrap_or_exit_with_error("Could not create database");
		fs::write(db_path, serde_json::to_string_pretty(&db).unwrap())
			.unwrap_or_exit_with_error("Could not initialize database");
	}

	fn ensure() {
		if !Database::exists() { Database::init() }
	}

	pub fn instance() -> Database {
		Database::ensure();

		let db = fs::read_to_string(Database::get_path())
			.expect("Getting database failed");

		serde_json::from_str(&db)
			.unwrap_or_exit_with_error("Corrupted database file")
	}

	pub fn get_song_names(&self) -> Vec<String> {
		self.songs.iter().map(|song| song.name.clone()).collect()
	}

	pub fn get_artist_names(&self) -> Vec<String> {
		self.songs.iter().map(|song| song.artists.clone()).flatten().collect()
	}

	pub fn get_tags(&self) -> Vec<String> {
		self.songs.iter().map(|song| song.tags.clone()).flatten().collect()
	}

	pub fn get_next_id(&self) -> u64 {
		self.songs.iter().map(|song| song.id).max().unwrap_or_default() + 1
	}
	
	pub fn get_song_by_name(&self, name: &str) -> &Song {
		&self.songs.iter().find(|song| song.name == name)
			.unwrap_or_exit_with_error("Invalid song name")
	}

	pub fn add_song(&mut self, song: Song){
		let db_path = Database::get_path();
		self.songs.push(song);
		fs::write(db_path, serde_json::to_string_pretty(self).unwrap())
			.unwrap("Could not update database");
	}
}

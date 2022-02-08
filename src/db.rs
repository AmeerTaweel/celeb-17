// Database

use crate::io;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{ env, fs, path::PathBuf };

#[derive(Serialize, Deserialize)]
struct Song {
	id: u64,
	name: String,
	artists: Vec<String>,
	tags: Vec<String>,
	version_of: u64
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
struct Database {
	music_dir_location: String,
	songs: Vec<Song>
}

pub fn get_path() -> PathBuf {
	let db_path = env::var("XDG_CONFIG_HOME")
		.expect("Getting database path failed");

	let db_path = format!("{}/celeb-17/db.json", db_path);
	
	PathBuf::from(db_path)
}

pub fn exists() -> bool {
	get_path().is_file()
}

pub fn init() {
	let music_dir_location = io::prompt("Where is your music directory located?", &vec![]);
	let db_path = get_path();
	let db_dir = db_path.parent().unwrap();
	fs::create_dir_all(db_dir).unwrap();
	let db = Database { music_dir_location, songs: vec![] };
	fs::write(db_path, serde_json::to_string_pretty(&db).unwrap()).unwrap();
}

pub fn ensure() {
	if !exists() { init() }
}

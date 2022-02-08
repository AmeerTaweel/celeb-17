// Database

use std::{ env, fs, path::PathBuf };

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
	let db_path = get_path();
	let db_dir = db_path.parent().unwrap();
	fs::create_dir_all(db_dir).unwrap();
	fs::write(db_path, "{}").unwrap();
}

pub fn ensure() {
	if !exists() { init() }
}

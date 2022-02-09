use colored::Colorize;

pub fn error(msg: &str){
	println!(
		"{} {}",
		format!("FATAL ERROR:").red().bold(),
		format!("{}.", msg).red()
	);
}

pub fn warn(msg: &str){
	println!(
		"{} {}",
		format!("WARNING:").yellow().bold(),
		format!("{}.", msg).yellow()
	);
}

pub fn info(msg: &str){
	println!(
		"{} {}",
		format!("INFO:").blue().bold(),
		format!("{}.", msg)
	);
}

pub fn success(msg: &str){
	println!(
		"{} {}",
		format!("SUCCESS:").green().bold(),
		format!("{}.", msg).green()
	);
}

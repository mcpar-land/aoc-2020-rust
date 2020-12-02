use std::fs;

/// Read a file from the `./input` directory
pub fn read_input(name: &str) -> String {
	let file_path = format!("./input/{}", name);
	fs::read_to_string(&file_path).unwrap()
}

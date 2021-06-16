use std::process;
use list_txt;


fn main() {
	let files: Vec<String> = list_txt::log_path().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
	println!("{:?}", files);
}
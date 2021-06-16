use std::env;
use log_analysis::{
	grep, grep::Config,
	files_handling::{
		untar, list_files_log
	}
};
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if let Err(e) = untar::untar(&args) {
        eprintln!("Problem parsing arguments: {}", e);

        process::exit(1);
    }
	
	let files: Vec<String> = list_files_log::log_path().unwrap_or_else(|err| {
        eprintln!("Problem log files: {}", err);
        process::exit(1);
    });
	
	let mut args_iter = args.iter();
	args_iter.next(); //skip first value of args : name function
	args_iter.next(); //skip second value of args : filesName.tar.gz
	
	let config = Config::new(args_iter, files).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
	
	if let Err(e) = grep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

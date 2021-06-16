use std::env;
use untar_log::untar;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if let Err(e) = untar(&args) {
        eprintln!("Problem parsing arguments: {}", e);

        process::exit(1);
    }   
}

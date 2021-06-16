use std::fs;

///Create a Vec<String> with the list of log files from ./log
pub fn log_path() -> Result<Vec<String>, &'static str> {
	// initialise path with log directory content. And check if it worked.
	if let Ok(paths) = fs::read_dir("./log") {
		//transform path content into a Vec<string> and filter to keep only .log
		let results : Vec<String> = paths.map(
							|x| x.unwrap().path().display().to_string())
							.filter(|x| x.ends_with(".log"))
							.collect();
		//check if there are log in results					
		if results.len() > 0{
			Ok(results)
		}
		else{
			return Err("There is no log in the archive")
		}
	}
	else{
		return Err("There is no such directory (daemon prolem)")
	}
}
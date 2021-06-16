use jsonrpc_core::{Error, ErrorCode, IoHandler, Params, Value, serde_json::json};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
use serde::Deserialize;
use server_log_analysis::{
	grep, grep::Config,
	files_handling::{
		untar, list_files_log
	}
};

// =============================================================================
//function for returning cURL error message
fn make_error (error: &str) -> Error {
  Error { code: ErrorCode::ServerError(0), message: error.to_string(), data: None }
}

// =============================================================================
//function to analyse file or tarball
fn analysis(query: String, file: String) -> Result<(), &'static str>{
	let mut files: Vec<String> = Vec::new();//var to check log
	if file.ends_with(".tar.gz") { //check if archives
		if let Err(e) = untar::untar(file) { // send the content to ./log
			return Err(e);
		}
		
		let mut list_files : Vec<String> = match list_files_log::log_path(){ // make a list of log files in ./log
			Ok(list) => list,
			Err(e) => {
				return Err(e)
			}
		};
		
		files.append(&mut list_files);// add list
	}
	else{
		files.push(file);//add file_name
	}
	
	let config = match Config::new(query, files) { 
		Ok(conf) => conf,
		Err(e) => {
			return Err(e);
		}
	};	
	
	if let Err(e) = grep::run(config) {
		return Err(e);
	}
	
	Ok(()) // no return value at the moment, should send back files with lines that contains query
}

fn main () {

	let mut io = IoHandler::new(); // create the thread that will serve for method and listening
	
	// method that will call the analysis part
	io.add_method("LogAnalysis", |params: Params| {
		#[derive(Deserialize)]
		struct LogAnalysis {
		  file_name: String,
		  query : String
		}
		
		let parsed: LogAnalysis = params.parse()?;
		
		match analysis(parsed.query, parsed.file_name){
			Ok(_) => Ok(Value::String(String::from("succes"))),
			Err(e) => Err(make_error(e))
		}
		
		/*
		while worked{
			let parsed: LogAnalysis = params.parse()?;
			if parsed.file_name.ends_with(".log") {
				
				let config = match Config::new(parsed.query, vec![parsed.file_name]) {
				  Ok(conf) => conf,
				  Err(e) => {
					  make_error(&format!("{}{}", "Problem parsing arguments: ", e).to_string());
					  break;
				  }
				};
				
				/*
				let config = Config::new(parsed.query, vec![parsed.file_name]).unwrap_or_else(|err| {
					Err(make_error(&format!("{}{}", "Problem parsing arguments: ", err).to_string()));
					process::exit(1);
				});
				*/
				
				if let Err(e) = grep::run(config) {
					make_error(&format!("{}{}", "Application error: ", e.to_string()).to_string());
					worked = false;
					break;
				}
			}
			break;
		}
		if worked{
			Ok(Value::String(String::from("succes")))
		}else{
			Err(make_error("failed"))
		}
		*/
	});
	
	//server that will listen on port 3030 with any ip.
  let server = ServerBuilder::new(io)
    .threads(2)
    .rest_api(RestApi::Unsecure)
    .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
    .start_http(&"0.0.0.0:3030".parse().unwrap())
    .expect("Unable to start RPC server");
	
  server.wait(); // for the thread to end to shut down. There it will never stop.
}
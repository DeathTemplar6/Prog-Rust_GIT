use bytes::BufMut;
use futures::TryStreamExt;
use std::convert::Infallible;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};
use log_analysis_server_side::{
	grep, grep::Config,
	files_handling::{
		untar, list_files_log
	}
};
//use std::str;

#[tokio::main]
async fn main() {
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(upload);
        
    let download_route = warp::path("files").and(warp::fs::dir("./files/"));

    let router = upload_route.or(download_route).recover(handle_rejection);
    println!("Server started at localhost:3030");
    warp::serve(router).run(([0, 0, 0, 0], 3030)).await;
}


async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

	let mut cpt_file : i8 = 1;
    for p in parts {
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "application/octet-stream" => {
                        file_ending = "tar.gz";
                    }
                    "text/plain" => {
                        file_ending = "log";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }
		
			let name = match p.filename(){
				Some(x) => x.to_string(),
				None => format!("{}{}.{}", "file".to_string(), cpt_file, file_ending),
			};
			
			let cpt_file = cpt_file + 1;


            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;
	    
            let file_name = format!("./files/{}", name);

            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", file_name);
        } 
    }
    //let querries: Vec<_> = querry_bin.iter().map(|x| str::from_utf8(&x).unwrap()).collect();   
    
    //let mut test = Vec::new();
    //test.push(querries);
    //test.push(vec!["a", "b", "c"]);

    //Ok(warp::reply::json(&test))
	Ok("succes")
}


async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

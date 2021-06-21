use bytes::BufMut;
use futures::TryStreamExt;
use std::convert::Infallible;
use uuid::Uuid;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};
use std::str;

#[tokio::main]
async fn main() {
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(upload);
        
    let download_route = warp::path("files").and(warp::fs::dir("./files/"));

    let router = upload_route.or(download_route).recover(handle_rejection);
    println!("Server started at localhost:8080");
    warp::serve(router).run(([0, 0, 0, 0], 8080)).await;
}


async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    //let mut querries = Vec::new();
    let mut querry_bin = Vec::new();
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
	    	None => format!("{}.{}", Uuid::new_v4().to_string(), file_ending),
	    };


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
        } else if p.name() == "querry" {
        
        	let mut fail = true;
        	match gather_binary(p) {
        		Ok(res) => querry_bin.push(res),
        		Err(e) => fail = false,
        	}
        	if fail == false{
        		warp::reject::reject();
        	}
        	
        	/*
         	querry_bin.push(p.stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?);
                */          	
        }
    }
    let querries: Vec<_> = querry_bin.iter().map(|x| str::from_utf8(&x).unwrap()).collect();   
    
    let mut test = Vec::new();
    test.push(querries);
    test.push(vec!["a", "b", "c"]);

    Ok(warp::reply::json(&test))
}

async fn gather_binary(p : Part) -> Result<Vec<u8>,Rejection> {
	p.stream()
        .try_fold(Vec::new(), |mut vec, data| {
        	vec.put(data);
                async move { Ok(vec) }
         })
         .await
         .map_err(|e| {
                 eprintln!("reading file error: {}", e);
                 warp::reject::reject()
         })?
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

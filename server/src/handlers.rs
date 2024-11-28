use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use std::convert::Infallible;
use uuid::Uuid;
use warp::{http::StatusCode, multipart::FormData, Rejection, Reply};
use prover_lib;

pub(super) async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let mut parts = form.into_stream();
    println!("parts: {:#?}", parts);
    while let Some(Ok(p)) = parts.next().await {
        if p.name() == "file" {
            println!("part {:#?}", p);
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "application/pdf" => {
                        file_ending = "pdf";
                    }
                    "image/png" => {
                        file_ending = "png";
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

            // let file_name = format!("./files/{}.{}", Uuid::new_v4(), file_ending);
            // tokio::fs::write(&file_name, value).await.map_err(|e| {
            //     eprint!("error writing file: {}", e);
            //     warp::reject::reject()
            // })?;
            // println!("created file: {}", file_name);

            let result = prover_lib::run(&value);

            println!("result: {}", result);
        }
    }

    Ok("success")
}

pub(super) async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
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
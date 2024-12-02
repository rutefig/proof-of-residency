use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use prover_lib;
use std::convert::Infallible;
use warp::{http::StatusCode, multipart::FormData, Rejection, Reply};

pub(super) async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let mut parts = form.into_stream();

    while let Some(Ok(p)) = parts.next().await {
        if p.name() == "file" {
            let content_type = p.content_type();
            match content_type {
                // for now only pdf files are supported
                Some(file_type) => match file_type {
                    "application/pdf" => {
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

                        let result = prover_lib::run(&value);

                        println!("result: {}", result);

                        return Ok(warp::reply::json(&result));
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
        }
    }

    Ok(warp::reply::json(&"success"))
}


// TODO: handle email login with zk email

pub(super) async fn handle_rejection(
    err: Rejection,
) -> std::result::Result<impl Reply, Infallible> {
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

mod ashcon;

use warp::{Filter, Reply};

use image::imageops;

use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Request-Method", "Content-Type"])
        .allow_methods(vec!["GET"]);

    let routes = skin_api().with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug, serde::Deserialize)]
pub struct ProfileOptions {
    username: String,
    download: Option<bool>,
}

pub fn skin_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("profile")
        .and(warp::get())
        .and(warp::query::<ProfileOptions>())
        .and_then(get_skin)
}

pub async fn get_skin(options: ProfileOptions) -> Result<impl warp::Reply, Infallible> {
    if let Ok(profile) = ashcon::get_profile(&options.username).await {
        let buffer = ashcon::get_skin_bytes(&profile.textures.skin.url)
            .await
            .unwrap();
        let mut skin = image::load_from_memory(&buffer).unwrap();
        let mut base_skin = image::open("diamonddagger.png").unwrap();

        let overlay = skin.crop(0, 0, 64, 16);
        imageops::overlay(&mut base_skin, &overlay, 0, 0);

        let mut skin_buffer = Vec::new();
        base_skin
            .write_to(&mut skin_buffer, image::ImageFormat::Png)
            .unwrap();

        let mut response = warp::reply::Response::new(skin_buffer.into());
        response.headers_mut().insert(
            "Content-Type",
            warp::http::header::HeaderValue::from_static("image/png"),
        );

        if let Some(download) = options.download {
            if download {
                response.headers_mut().insert(
                    "Content-Disposition",
                    warp::http::header::HeaderValue::from_static(
                        "attachment; filename=\"skin.png\"",
                    ),
                );
            }
        }

        return Ok(response);
    }

    let error = vec!["error: shit wrong name dude"];

    Ok(warp::reply::json(&error).into_response())
}

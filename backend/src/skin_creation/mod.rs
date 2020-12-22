use image::imageops;
use rocket::http::Status;

use crate::api::APIError;

const BASE_SKIN_PATH: &'static str = "./diamonddagger.png";

pub async fn diamondagify(uuid: &str) -> Result<String, APIError> {
    let overlay_path = format!("./skins/cache/{}.png", uuid);
    let image_open = image::open(BASE_SKIN_PATH);

    match image_open {
        Ok(mut base_skin) => {
            let skin_open = image::open(&overlay_path);
            match skin_open {
                Ok(mut skin) => {
                    let mut overlay = skin.crop(0, 0, 64, 16);

                    imageops::overlay(&mut base_skin, &mut overlay, 0, 0);
                    let out_path = format!("./skins/modified/{}.png", uuid);

                    match base_skin.save(&out_path) {
                        Ok(_) => return Ok(out_path),
                        Err(err) => {
                            return Err(APIError(format!("{}", err), Status::InternalServerError));
                        }
                    }
                }
                Err(err) => {
                    return Err(APIError(format!("{}", err), Status::InternalServerError));
                }
            }
        }
        Err(err) => {
            return Err(APIError(format!("{}", err), Status::InternalServerError));
        }
    }
}

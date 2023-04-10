use std::io::Write;

use crate::state::r#struct as stcState;
use sanitize_filename as sf;
use actix_web as aw;
use futures_util as fu;
use fu::StreamExt;

use common::commands::file_commands::UploadFile;

pub async fn upload_file(app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<UploadFile>, mut payload: aw::web::Payload) -> impl aw::Responder {
    let query = query.into_inner();

    let mut bytes = aw::web::BytesMut::new();

    while let Some(Ok(item)) = payload.next().await {
        bytes.extend_from_slice(&item);
    }

    let file_name = sf::sanitize(query.file_name);

    if !bytes.is_empty()  {
        let file_path = app_state.public_directory.join(file_name);

        let mut file = std::fs::File::create(file_path)
        .unwrap_or_else(|error| {
            log::error!("Error occured during creating file.");
            log::error!("{error:?}");
            panic!()
        });

        file.write_all(&bytes)
        .unwrap_or_else(|error| {
            log::error!("Error occured during writing bytes over file.");
            log::error!("{error:?}");
            panic!()
        });

        return aw::HttpResponse::Ok().finish()
    }

    aw::HttpResponse::BadRequest().finish()
}
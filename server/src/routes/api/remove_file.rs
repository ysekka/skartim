use sanitize_filename as sf;
use actix_web as aw;

use common::commands::file_commands::RemoveFile;
use crate::state::r#struct as stcState;

pub async fn remove_file(app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<RemoveFile>) -> impl aw::Responder {
    let query = query.into_inner();

    let file_name = sf::sanitize(query.file_name);
    let file_path = app_state.public_directory.join(file_name);

    if file_path.exists() {
        std::fs::remove_file(file_path)
        .unwrap_or_else(|error| {
            log::error!("Error occured during removing file.");
            log::error!("{error:?}");
            panic!()
        });

        return aw::HttpResponse::Ok().finish()
    }

    aw::HttpResponse::NotFound().finish()
}
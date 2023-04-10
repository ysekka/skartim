use sanitize_filename as sf;
use actix_files as af;
use actix_web as aw;

use common::commands::file_commands::GetFile;
use crate::state::r#struct as stcState;

pub async fn serve_file(app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<GetFile>) -> impl aw::Responder {
    let query = query.into_inner();

    let file_name = sf::sanitize(query.file_name);
    let file_path = app_state.public_directory.join(file_name);

    if file_path.exists() {
        return Some(af::NamedFile::open_async(&file_path).await.unwrap())
    }

    None
}
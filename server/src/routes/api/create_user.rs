use actix_web as aw;

use common::commands::user_commands::CreateUser;
use entity::sea_orm::{ActiveValue, ActiveModelTrait};
use sha2::Digest;
use crate::state::r#struct as stcState;
use entity::sea_orm_active_enums as soae;

pub async fn create_user(user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<CreateUser>) -> impl aw::Responder {
    let query = query.into_inner();

    if (query.user_status == soae::UserStatus::Administrator || query.user_status == soae::UserStatus::Coadministrator) && user.user_status == soae::UserStatus::Coadministrator {
        return aw::HttpResponse::Forbidden().finish()
    }

    let mut hasher = sha2::Sha256::new();

    hasher.update(query.user_password);

    let active_user = entity::users_table::ActiveModel {
        user_uuid: ActiveValue::NotSet,
        user_email: ActiveValue::Set(query.user_email),
        user_status: ActiveValue::Set(query.user_status),
        user_password: ActiveValue::Set(hasher.finalize().iter().map(|byte| *byte).collect::<Vec<_>>()),
        user_realname: ActiveValue::Set(query.user_realname),
    };

    let created_user = active_user.insert(&app_state.database_connection).await.unwrap();

    return aw::HttpResponse::Ok().json(created_user)
}
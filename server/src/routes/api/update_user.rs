use actix_web as aw;
use entity::sea_orm::{EntityTrait, IntoActiveModel, ActiveValue};
use sha2::Digest;

use crate::state::r#struct as stcState;
use common::commands::user_commands::UpdateUser;
use entity::sea_orm_active_enums as soae;

pub async fn update_user(user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<UpdateUser>) -> impl aw::Responder {
    let query = query.into_inner();

    let user_query = entity::users_table::Entity::find_by_id(query.user_uuid)
    .one(&app_state.database_connection).await.unwrap();

    if let Some(queried_user) = user_query {
        if !(user.user_status == soae::UserStatus::Administrator || user.user_uuid == queried_user.user_uuid) {
            return aw::HttpResponse::Forbidden().finish();
        }

        let mut active_user = queried_user.into_active_model();

        if let Some(user_realname) = query.user_realname {
            active_user.user_realname = ActiveValue::Set(user_realname)
        }

        if let Some(user_email) = query.user_email {
            active_user.user_email = ActiveValue::Set(user_email)
        }

        if let Some(user_password) = query.user_password {
            let mut hasher = sha2::Sha256::new();

            hasher.update(user_password);

            active_user.user_password = ActiveValue::Set(hasher.finalize().iter().map(|byte| *byte).collect::<Vec<_>>())
        }
    }

    aw::HttpResponse::NotFound().finish()
}
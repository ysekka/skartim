use actix_web as aw;

use common::commands::user_commands::RemoveUser;
use entity::sea_orm::EntityTrait;
use crate::state::r#struct as stcState;
use entity::sea_orm_active_enums as soae;

pub async fn remove_user(user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<RemoveUser>) -> impl aw::Responder {
    let query = query.into_inner();

    let user_query = entity::users_table::Entity::find_by_id(query.user_uuid)
    .one(&app_state.database_connection).await.unwrap();

    if let Some(queried_user) = user_query {
        if (user.user_status == soae::UserStatus::Coadministrator) && (queried_user.user_status == soae::UserStatus::Administrator) {
            return aw::HttpResponse::Forbidden().finish()
        }

        let rows = entity::users_table::Entity::delete_by_id(query.user_uuid)
        .exec(&app_state.database_connection).await.unwrap();

        if rows.rows_affected != 0 {
            return aw::HttpResponse::Ok().finish()
        }
    }

    aw::HttpResponse::NotFound().finish()
}
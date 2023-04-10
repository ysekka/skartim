use actix_web as aw;

use common::commands::user_commands::GetUser;
use entity::sea_orm::EntityTrait;
use crate::state::r#struct as stcState;
use entity::sea_orm_active_enums as soae;

pub async fn query_user(user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<GetUser>) -> impl aw::Responder {
    let query = query.into_inner();

    let user_query = entity::users_table::Entity::find_by_id(query.user_uuid)
    .one(&app_state.database_connection).await.unwrap();

    if let Some(queried_user) = user_query {
        if (queried_user.user_status == soae::UserStatus::Administrator || queried_user.user_status == soae::UserStatus::Coadministrator) && user.user_status == soae::UserStatus::Coadministrator {
            return aw::HttpResponse::Forbidden().finish()
        }

        return aw::HttpResponse::Ok().json(queried_user)
    }

    aw::HttpResponse::NotFound().finish()
}
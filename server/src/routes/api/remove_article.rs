use actix_web as aw;
use entity::sea_orm::EntityTrait;

use crate::state::r#struct as stcState;
use common::commands::article_commands::RemoveArticle;
use entity::sea_orm_active_enums as soae;

pub async fn remove_article(user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<RemoveArticle>) -> impl aw::Responder {
    let query = query.into_inner();

    let article_query = entity::articles_table::Entity::find_by_id(query.article_uuid)
    .one(&app_state.database_connection).await.unwrap();

    if let Some(article) = article_query {
        if user.user_status == soae::UserStatus::Author && article.article_author != user.user_uuid {
            return aw::HttpResponse::Forbidden().finish()
        }

        let deletion = entity::articles_table::Entity::delete_by_id(query.article_uuid)
        .exec(&app_state.database_connection).await.unwrap();

        if deletion.rows_affected != 0 {
            return aw::HttpResponse::Ok().finish()
        } 
    }

    aw::HttpResponse::NotFound().finish()
}
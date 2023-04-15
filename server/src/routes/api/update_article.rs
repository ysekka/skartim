use actix_web as aw;

use common::commands::article_commands::UpdateArticle;
use entity::sea_orm::{EntityTrait, IntoActiveModel, ActiveValue, ActiveModelTrait};
use entity::sea_orm_active_enums as soae;
use crate::state::r#struct as stcState;

pub async fn update_article(user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<UpdateArticle>) -> impl aw::Responder {
    let query = query.into_inner();

    let article_query = entity::articles_table::Entity::find_by_id(query.article_uuid)
    .one(&app_state.database_connection).await.unwrap();

    if let Some(article) = article_query {
        if user.user_status == soae::UserStatus::Author && article.article_author != user.user_uuid {
            return aw::HttpResponse::Forbidden().finish()
        }

        let mut active_article = article.into_active_model();

        if let Some(article_title) = query.article_title {
            active_article.article_title = ActiveValue::Set(article_title)
        }

        if let Some(article_content) = query.article_content {
            active_article.article_content = ActiveValue::Set(article_content)
        }

        if let Some(article_thumbnail) = query.article_thumbnail {
            active_article.article_content = ActiveValue::Set(article_thumbnail)
        }

        if let Some(article_visibility) = query.article_visibility {
            active_article.article_visibility = ActiveValue::Set(article_visibility)
        }

        if let Some(article_tags) = query.article_tags {
            active_article.article_tags = ActiveValue::Set(Some(article_tags.split("|").map(|part| part.trim().to_uppercase().to_owned()).collect::<Vec<String>>()))
        }

        let updation = active_article.update(&app_state.database_connection).await.unwrap();

        return aw::HttpResponse::Ok().json(updation)
    }

    aw::HttpResponse::NotFound().finish()
}
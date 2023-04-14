use actix_web as aw;

use common::commands::article_commands::CreateArticle;
use entity::sea_orm::ActiveModelTrait;
use migration::sea_orm::ActiveValue;
use entity::sea_orm_active_enums as soae;

use crate::state::r#struct as stcState;

pub async fn create_article(user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<CreateArticle>) -> impl aw::Responder {
    let query = query.into_inner();

    if (user.user_status == soae::UserStatus::Author || user.user_status == soae::UserStatus::Highauthor) && (query.article_type == soae::ArticleType::Announcement) {
        return aw::HttpResponse::Forbidden().finish();
    }

    let active_article = entity::articles_table::ActiveModel {
        article_uuid: ActiveValue::NotSet,
        article_views: ActiveValue::NotSet,
        article_title: ActiveValue::Set(query.article_title),
        article_author: ActiveValue::Set(user.user_uuid),
        article_content: ActiveValue::Set(query.article_content),
        article_thumbnail: ActiveValue::Set(query.article_thumbnail),
        article_type: ActiveValue::Set(query.article_type),
        article_timestamp: ActiveValue::NotSet,
        article_visibility: ActiveValue::Set(match query.article_visibility {
            Some(article_visibility) => article_visibility,
            None => true,
        })
    };

    let new_article = active_article.insert(&app_state.database_connection).await.unwrap();

    return aw::HttpResponse::Ok().json(new_article);
}
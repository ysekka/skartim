use actix_web as aw;

use common::commands::article_commands::{QueryArticles, QueryArticlesPrivate};
use entity::sea_orm::{JoinType, EntityTrait, QuerySelect, QueryFilter, ColumnTrait, RelationTrait, IntoActiveModel, ActiveValue, ActiveModelTrait};
use crate::state::r#struct as stcState;

pub async fn query_articles(app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<QueryArticles>) -> Option<impl aw::Responder> {
    let query = query.into_inner();

    match query.article_uuid {
        Some(article_uuid) => {
            let article_query = entity::articles_table::Entity::find_by_id(article_uuid)
            .filter(entity::articles_table::Column::ArticleVisibility.eq(true))
            .one(&app_state.database_connection).await.unwrap();

            if let Some(article) = article_query {
                let views = article.article_views;
                let mut active_article = article.into_active_model();

                active_article.article_views = ActiveValue::Set(views + 1);

                active_article.update(&app_state.database_connection).await.unwrap();
            }

            entity::articles_table::Entity::find_by_id(article_uuid)
            .filter(entity::articles_table::Column::ArticleVisibility.eq(true))
            .join(JoinType::InnerJoin, entity::articles_table::Relation::UsersTable.def())
            .select_only()
            .columns([
                entity::articles_table::Column::ArticleUuid,
                entity::articles_table::Column::ArticleTimestamp,
                entity::articles_table::Column::ArticleThumbnail,
                entity::articles_table::Column::ArticleType,
                entity::articles_table::Column::ArticleViews,
                entity::articles_table::Column::ArticleTitle,
                entity::articles_table::Column::ArticleContent
            ])
            .columns([
                entity::users_table::Column::UserRealname,
                entity::users_table::Column::UserUuid,
            ])
            .into_json()
            .one(&app_state.database_connection)
            .await.unwrap()
            .map(|value| aw::HttpResponse::Ok().json(value))
        },
        
        None => {
            let mut articles_query = entity::articles_table::Entity::find()
            .filter(entity::articles_table::Column::ArticleVisibility.eq(true))
            .join(JoinType::InnerJoin, entity::articles_table::Relation::UsersTable.def())
            .select_only()
            .columns([
                entity::articles_table::Column::ArticleUuid,
                entity::articles_table::Column::ArticleTimestamp,
                entity::articles_table::Column::ArticleThumbnail,
                entity::articles_table::Column::ArticleType,
                entity::articles_table::Column::ArticleViews,
                entity::articles_table::Column::ArticleTitle,
                entity::articles_table::Column::ArticleContent
            ])
            .columns([
                entity::users_table::Column::UserRealname,
                entity::users_table::Column::UserUuid
            ]);

            if let Some(article_title) = query.article_title {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleTitle.contains(article_title.as_str()))
            }

            if let Some(article_content) = query.article_content {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleContent.contains(article_content.as_str()))
            }

            if let Some(article_author) = query.article_author {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleAuthor.eq(article_author))
            }

            if let Some(article_timestamp) = query.article_timestamp {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleTimestamp.eq(article_timestamp))
            }

            if let Some(limit) = query.limit {
                articles_query = articles_query.limit(limit as u64)
            }

            let articles_query = articles_query
            .into_json()
            .all(&app_state.database_connection)
            .await.unwrap();

            if !articles_query.is_empty() {
                return Some(aw::HttpResponse::Ok().json(articles_query))
            }

            None
        }
    }
}

pub async fn query_articles_private(app_state: aw::web::Data<stcState::AppState>, query: aw::web::Query<QueryArticlesPrivate>) -> Option<impl aw::Responder> {
    let query = query.into_inner();

    match query.article_uuid {
        Some(article_uuid) => {
            entity::articles_table::Entity::find_by_id(article_uuid)
            .join(JoinType::InnerJoin, entity::articles_table::Relation::UsersTable.def())
            .column(entity::users_table::Column::UserRealname)
            .into_json()
            .one(&app_state.database_connection)
            .await.unwrap()
            .map(|value| aw::HttpResponse::Ok().json(value))
        }

        None => {
            let mut articles_query = entity::articles_table::Entity::find()
            .filter(entity::articles_table::Column::ArticleVisibility.eq(true))
            .column(entity::users_table::Column::UserRealname);

            if let Some(article_title) = query.article_title {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleTitle.contains(article_title.as_str()))
            }

            if let Some(article_content) = query.article_content {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleContent.contains(article_content.as_str()))
            }

            if let Some(article_author) = query.article_author {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleAuthor.eq(article_author))
            }

            if let Some(article_timestamp) = query.article_timestamp {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleTimestamp.eq(article_timestamp))
            }
            
            if let Some(article_visibility) = query.article_visibility {
                articles_query = articles_query.filter(entity::articles_table::Column::ArticleVisibility.eq(article_visibility))
            }

            if let Some(limit) = query.limit {
                articles_query = articles_query.limit(limit as u64)
            }

            let articles_query = articles_query
            .into_json()
            .all(&app_state.database_connection)
            .await.unwrap();

            if !articles_query.is_empty() {
                return Some(aw::HttpResponse::Ok().json(articles_query))
            }

            None
        }
    }
}
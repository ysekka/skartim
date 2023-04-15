use entity::sea_orm_active_enums as soae;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryArticles {
    pub article_uuid: Option<uuid::Uuid>,
    pub article_tag: Option<String>,
    pub article_title: Option<String>,
    pub article_content: Option<String>,
    pub article_author: Option<uuid::Uuid>,
    pub article_timestamp: Option<chrono::NaiveDateTime>,
    pub article_type: Option<soae::ArticleType>,
    pub limit: Option<usize>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryArticlesPrivate {
    pub article_uuid: Option<uuid::Uuid>,
    pub article_tag: Option<String>,
    pub article_title: Option<String>,
    pub article_content: Option<String>,
    pub article_author: Option<uuid::Uuid>,
    pub article_timestamp: Option<chrono::NaiveDateTime>,
    pub article_visibility: Option<bool>,
    pub article_type: Option<soae::ArticleType>,
    pub limit: Option<usize>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateArticle {
    pub article_title: String,
    pub article_content: String,
    pub article_tags: Option<String>,
    pub article_thumbnail: Option<String>,
    pub article_type: soae::ArticleType,
    pub article_visibility: Option<bool>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateArticle {
    pub article_uuid: uuid::Uuid,
    pub article_title: Option<String>,
    pub article_content: Option<String>,
    pub article_thumbnail: Option<String>,
    pub article_visibility: Option<bool>,
    pub article_tags: Option<String>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveArticle {
    pub article_uuid: uuid::Uuid,
}
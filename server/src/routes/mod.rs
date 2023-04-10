use actix_web_httpauth as awh;
use actix_web as aw;

pub mod validation;
pub mod catcher;

pub mod api {
    pub mod self_query;

    pub mod create_article;
    pub mod update_article;
    pub mod query_articles;
    pub mod remove_article;

    pub mod create_user;
    pub mod update_user;
    pub mod remove_user;
    pub mod query_user;

    pub mod upload_file;
    pub mod remove_file;
    pub mod serve_file;
}

pub fn configure(configurations: &mut aw::web::ServiceConfig) {
    use crate::routes::validation::validation;
    use crate::routes::api::self_query;
    use crate::routes::api::query_articles;
    use crate::routes::api::create_article;
    use crate::routes::api::update_article;
    use crate::routes::api::remove_article;

    use crate::routes::api::remove_user;
    use crate::routes::api::update_user;
    use crate::routes::api::create_user;
    use crate::routes::api::query_user;

    use crate::routes::api::upload_file;
    use crate::routes::api::remove_file;
    use crate::routes::api::serve_file;

    configurations
    .route("/public", aw::web::get().to(serve_file::serve_file))
    .service(
        aw::web::scope("/api")
        .route("/article", aw::web::get().to(query_articles::query_articles))
        .service(
            aw::web::scope("/private")
            .wrap(awh::middleware::HttpAuthentication::bearer(validation))
            .route("/user/self", aw::web::get().to(self_query::self_query))
            .route("/article", aw::web::get().to(query_articles::query_articles_private))
            .route("/article", aw::web::put().to(update_article::update_article))
            .route("/article", aw::web::post().to(create_article::create_article))
            .route("/article", aw::web::delete().to(remove_article::remove_article))
            .route("/file", aw::web::post().to(upload_file::upload_file))
            .route("/file", aw::web::delete().to(remove_file::remove_file))
            .service(
                aw::web::scope("/administration")
                .guard(aw::guard::fn_guard(|context| {
                    use entity::sea_orm_active_enums as soae;
    
                    let request_data = context.req_data();
                    let user = request_data.get::<entity::users_table::Model>().unwrap();
    
                    user.user_status == soae::UserStatus::Administrator || user.user_status == soae::UserStatus::Coadministrator
                }))
                .route("/user", aw::web::get().to(query_user::query_user))
                .route("/user", aw::web::put().to(update_user::update_user))
                .route("/user", aw::web::post().to(create_user::create_user))
                .route("/user", aw::web::delete().to(remove_user::remove_user))
            ),
        ),
    );
}

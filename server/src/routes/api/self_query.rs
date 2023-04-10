use actix_web as aw;

pub async fn self_query(user: aw::web::ReqData<entity::users_table::Model>) -> impl aw::Responder {
    return aw::HttpResponse::Ok().json(user.into_inner())
}
use actix_web_httpauth as awh;
use jsonwebtoken as jwt;
use serde_json as sj;
use actix_web as aw;
use aw::HttpMessage;

use entity::sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use sha2::Digest;

use crate::state::r#struct as stcState;
use common::paths::statics::PASSWORD_PATH;
use common::jwt_claims::Claims;

pub async fn validation(
    request: aw::dev::ServiceRequest,
    credentials: awh::extractors::bearer::BearerAuth,
) -> Result<aw::dev::ServiceRequest, (aw::Error, aw::dev::ServiceRequest)> {
    let secret_content =
        std::fs::read_to_string(PASSWORD_PATH.as_path()).unwrap_or_else(|error| {
            log::error!("Error occured during reading password file.");
            log::error!("{error}");
            std::process::exit(1)
        });

    let decoding_key = jwt::DecodingKey::from_secret(secret_content.as_bytes());
    let mut jwt_validator = jwt::Validation::new(jwt::Algorithm::HS256);

    jwt_validator.set_required_spec_claims::<String>(&[]);

    let claims = jwt::decode::<Claims>(credentials.token(), &decoding_key, &jwt_validator)
    .map_err(|error| aw::error::ErrorUnauthorized(error.to_string()));

    match claims {
        Ok(claims) => {
            let claims = claims.claims;

            let app_state = request.app_data::<aw::web::Data<stcState::AppState>>().unwrap();

            let mut hasher = sha2::Sha256::new();

            hasher.update(&claims.user_password);

            let user_query = entity::users_table::Entity::find_by_id(claims.user_uuid)
            .filter(entity::users_table::Column::UserPassword.eq(hasher.finalize().iter().map(|byte| *byte).collect::<Vec<u8>>()))
            .one(&app_state.database_connection).await.unwrap();

            if let Some(user) = user_query {
                request.extensions_mut().insert(user);

                return Ok(request)
            }
            
            Err((aw::error::ErrorUnauthorized(sj::json!({})), request))
        },
        
        Err(error) => Err((error, request))
    }
}
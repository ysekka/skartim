use serde_json as sj;
use actix_web as aw;

pub fn client_catcher<B>(service_response: aw::dev::ServiceResponse<B>) -> aw::Result<aw::middleware::ErrorHandlerResponse<B>> {
    let status = service_response.status();

    let (http_request, _) = service_response.into_parts();

    let new_response = aw::HttpResponseBuilder::new(status)
    .json(sj::json!({
        "status_code": status.as_u16(),
    }));

    let new_service_response = aw::dev::ServiceResponse::new(http_request, new_response.map_into_right_body());

    Ok(aw::middleware::ErrorHandlerResponse::Response(new_service_response))
}
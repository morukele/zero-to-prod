use actix_web::http::header::LOCATION;
use actix_web::web::Form;
use actix_web::HttpResponse;
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn login(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}

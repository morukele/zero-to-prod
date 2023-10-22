use actix_web::{http::header::ContentType, web, HttpResponse};
use sqlx::PgPool;

use crate::{authentication::get_username, session_state::TypedSession, utils::see_other};

// Returns an opaque 500 while preserving the error's root cause for logging.
fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub async fn admin_dashboard(
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &pool).await.map_err(e500)?
    } else {
        return Ok(see_other("/login"));
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta http-equiv="content-type" content="text/html; charset=utf-8">
                        <title>Admin dashboard</title>
                    </head>
                    <body>
                        <p>Welcome {username}!</p>
                        <p>Available actions:</p>
                        <ol>
                            <li><a href="/admin/password">Change password</a></li>
                            <li><a href="/admin/newsletters">Send newsletter issue</a></li>
                            <li>
                                <form name="logoutForm" action="/admin/logout" method="post">
                                    <input type="submit" value="Logout">
                                </form> 
                            </li>
                        </ol>
                    </body>
                </html>"#
        )))
}

mod auth;
mod user;

use actix_web::{web, HttpResponse};
use crate::errors::AppError;
use user::{create_user, me};
use auth::auth;

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let signup = web::resource("../html/signup").route(web::post().to(create_user));
    
    let auth = web::resource("../html/login").route(web::post().to(auth));

    let me = web::resource("../html/me")
        .route(web::get().to(me));

    config.service(signup).service(auth).service(me);
}

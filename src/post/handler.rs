use crate::database::Pool;
use crate::errors::ServiceError;
use crate::post::service as post;
use crate::user::model::{LoggedUser};

use crate::post::model::{PostData, PostDataEdit, PostDataStatusRq};
use actix_web::{web, HttpResponse};

pub async fn publish(
    post_data: web::Json<PostData>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
    match logged_user.0 {
        None => Ok(HttpResponse::Unauthorized().json(ServiceError::Unauthorized)),
        Some(user) => post::publish(user, post_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res)),
    }
}

pub async fn edit(
    put_data: web::Json<PostDataEdit>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
    match logged_user.0 {
        None => Ok(HttpResponse::Unauthorized().json(ServiceError::Unauthorized)),
        Some(user) => post::edit(user, put_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res)),
    }
}

pub async fn disable(
    put_data: web::Json<PostDataStatusRq>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
    match logged_user.0 {
        None => Ok(HttpResponse::Unauthorized().json(ServiceError::Unauthorized)),
        Some(ref _user) => { 
            has_role(logged_user, "admin")?;
            post::disable(put_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
        },
    }
}

pub async fn enable(
    put_data: web::Json<PostDataStatusRq>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, ServiceError> {
    match logged_user.0 {
        None => Ok(HttpResponse::Unauthorized().json(ServiceError::Unauthorized)),
        Some(ref _user) => { 
            has_role(logged_user, "admin")?;
            post::enable(put_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
        },
    }
}

pub fn has_role(logged_user: LoggedUser, role: &str) -> Result<bool, ServiceError> {
    match logged_user.0 {
        Some(ref user) if user.role == role => Ok(true),
        _ => Err(ServiceError::Unauthorized),
    }
}
use crate::database::Pool;
use crate::errors::ServiceError;
use crate::post::service as post;
use crate::user::model::{LoggedUser};

use crate::post::model::{PostData};
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
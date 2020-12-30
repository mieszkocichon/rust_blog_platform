mod handler;
pub mod model;
pub(crate) mod service;

use crate::post::handler::{publish, edit, disable, enable};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .service(web::resource("/publish").route(web::post().to(publish)))
            .service(web::resource("/edit").route(web::put().to(edit)))
            .service(web::resource("/disable").route(web::put().to(disable)))
            .service(web::resource("/enable").route(web::put().to(enable)))
    );
}

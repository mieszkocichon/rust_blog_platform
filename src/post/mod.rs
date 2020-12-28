mod handler;
pub mod model;
pub(crate) mod service;

use crate::post::handler::{publish};
use actix_web::web;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .service(web::resource("/publish").route(web::post().to(publish)))
    );
}

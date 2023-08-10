use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/courses", web::post().to(handle_post_new_course));
    cfg.route(
        "/courses/{tutor_id}",
        web::get().to(handle_get_courses_for_tutor),
    );
    cfg.route(
        "/courses/{tutor_id}/{course_id}",
        web::get().to(handle_get_course_details),
    );
}

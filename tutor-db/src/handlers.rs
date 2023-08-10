use super::errors::EzyTutorError;
use super::state::AppState;
use actix_web::{web, HttpResponse};

use super::models::course::NewCourse;

mod course;

use course::*;

pub async fn health_check_handler() -> HttpResponse {
    let response = format!("Alive");
    HttpResponse::Ok().json(&response)
}

pub async fn handle_get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    let db = &app_state.db;

    get_courses_for_tutor(tutor_id, &db)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn handle_get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    let db = &app_state.db;
    get_course_details(tutor_id, course_id, &db)
        .await
        .map(|record| HttpResponse::Ok().json(record))
}

pub async fn handle_post_new_course(
    data: web::Json<NewCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    let db = &app_state.db;

    post_new_course(data, &db)
        .await
        .map(|record| HttpResponse::Ok().json(record))
}

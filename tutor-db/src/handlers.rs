use super::models::{prelude::*, *};
use super::state::AppState;
use actix_web::{web, HttpResponse};
use sea_orm::*;

pub async fn health_check_handler() -> HttpResponse {
    let response = format!("Alive");
    HttpResponse::Ok().json(&response)
}

// pub async fn get_courses_for_tutor(tutor_id: i32) {

// }

pub async fn post_new_course(
    data: web::Json<NewCourse>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let db = &app_state.db;

    let new_course = course::ActiveModel {
        name: ActiveValue::Set(data.name.clone()),
        tutor_id: ActiveValue::Set(data.tutor_id),
        ..Default::default()
    };

    let rec = Course::insert(new_course.clone()).exec(db).await.unwrap();

    let rec = Course::find_by_id(rec.last_insert_id)
        .one(db)
        .await
        .unwrap();

    HttpResponse::Ok().json(rec)
}

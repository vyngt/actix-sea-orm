use super::models::{prelude::*, *};
use super::state::AppState;
use actix_web::{web, HttpResponse};
use sea_orm::*;

pub async fn health_check_handler() -> HttpResponse {
    let response = format!("Alive");
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let tutor_id = params.into_inner();
    let db = &app_state.db;
    let rec = Course::find()
        .filter(course::Column::TutorId.eq(tutor_id))
        .all(db)
        .await
        .unwrap();
    HttpResponse::Ok().json(rec)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let db = &app_state.db;
    let rec = Course::find()
        .filter(course::Column::TutorId.eq(tutor_id))
        .filter(course::Column::Id.eq(course_id))
        .one(db)
        .await;

    match rec {
        Ok(result) => match result {
            Some(record) => HttpResponse::Ok().json(record),
            None => HttpResponse::NotFound().json(String::from("Not found!")),
        },
        Err(_) => HttpResponse::InternalServerError().json(String::from("Something went wrong!!!")),
    }
}

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

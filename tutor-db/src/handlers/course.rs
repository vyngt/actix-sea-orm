use super::super::errors::EzyTutorError;
use super::super::models::{prelude::*, *};
use crate::models::course::Model;
use actix_web::web::Json;
use sea_orm::*;

pub async fn get_courses_for_tutor(
    tutor_id: i32,
    db: &DatabaseConnection,
) -> Result<Vec<Model>, EzyTutorError> {
    let rec = Course::find()
        .filter(course::Column::TutorId.eq(tutor_id))
        .all(db)
        .await?;

    match rec.len() {
        0 => Err(EzyTutorError::NotFound(
            "Courses not found in this Tutor".into(),
        )),
        _ => Ok(rec),
    }
}

pub async fn get_course_details(
    tutor_id: i32,
    course_id: i32,
    db: &DatabaseConnection,
) -> Result<Model, EzyTutorError> {
    let rec = Course::find()
        .filter(course::Column::TutorId.eq(tutor_id))
        .filter(course::Column::Id.eq(course_id))
        .one(db)
        .await?;

    match rec {
        Some(record) => Ok(record),
        None => Err(EzyTutorError::NotFound("Course not found".into())),
    }
}

pub async fn post_new_course(
    data: Json<NewCourse>,
    db: &DatabaseConnection,
) -> Result<Model, EzyTutorError> {
    let new_course = course::ActiveModel {
        name: ActiveValue::Set(data.name.clone()),
        tutor_id: ActiveValue::Set(data.tutor_id),
        ..Default::default()
    };

    let rec = Course::insert(new_course.clone()).exec(db).await;

    let last_id = match rec {
        Ok(r) => r.last_insert_id,
        Err(_) => 0,
    };

    let result = Course::find_by_id(last_id).one(db).await?;

    match result {
        Some(record) => Ok(record),
        None => Err(EzyTutorError::DBError("Can't not insert".into())),
    }
}

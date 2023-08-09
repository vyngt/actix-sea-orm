//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2
use actix_web::web;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "course")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tutor_id: i32,
    pub name: String,
    pub posted_time: Option<DateTime>,
}

impl From<web::Json<Model>> for Model {
    fn from(course: web::Json<Model>) -> Self {
        Model {
            id: course.id,
            tutor_id: course.tutor_id,
            name: course.name.clone(),
            posted_time: course.posted_time,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCourse {
    pub tutor_id: i32,
    pub name: String,
}

impl From<web::Json<NewCourse>> for NewCourse {
    fn from(course: web::Json<NewCourse>) -> Self {
        NewCourse {
            tutor_id: course.tutor_id,
            name: course.name.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

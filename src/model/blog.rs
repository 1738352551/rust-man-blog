use sea_orm::entity::prelude::*;
use serde::Serialize;


#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "blog")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub author: i32,
    pub view_count: Option<i32>,
    pub create_time: Option<DateTime>,
    pub create_by: Option<i32>,
    pub update_time: Option<DateTime>,
    pub update_by: Option<i32>,
    pub status: i8
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}
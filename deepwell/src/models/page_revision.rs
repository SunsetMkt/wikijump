//! SeaORM Entity. Generated by sea-orm-codegen 0.4.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "page_revision")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub revision_id: i64,
    pub page_id: Option<i32>,
    pub metadata_id: Option<i32>,
    pub flags: Option<String>,
    pub flag_text: bool,
    pub flag_title: bool,
    pub flag_file: bool,
    pub flag_rename: bool,
    pub flag_meta: bool,
    pub flag_new: bool,
    pub revision_number: Option<i32>,
    pub date_last_edited: Option<DateTime>,
    pub user_id: Option<i32>,
    pub user_string: Option<String>,
    pub comments: Option<String>,
    pub site_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::page_contents::Entity")]
    PageContents,
}

impl Related<super::page_contents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageContents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "page_category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub category_id: i64,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: Option<TimeDateTimeWithTimeZone>,
    pub site_id: i64,
    #[sea_orm(column_type = "Text")]
    pub slug: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::page::Entity")]
    Page,
    #[sea_orm(
        belongs_to = "super::site::Entity",
        from = "Column::SiteId",
        to = "super::site::Column::SiteId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Site,
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl Related<super::site::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Site.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

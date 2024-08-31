//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "advertisement")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub advertisement_id: i32,
    pub advertiser: String,
    pub content_html: String,
    pub is_active: i8,
    pub views: i32,
    pub clicks: i32,
    pub redirect_url: String,
    pub r#type: String,
    pub location: String,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

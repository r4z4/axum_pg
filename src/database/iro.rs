//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "iro")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub iro_id: i32,
    pub iro_name: String,
    pub iro_address_1: Option<String>,
    pub iro_address_2: Option<String>,
    pub iro_zip: Option<String>,
    pub iro_contact_f_name: Option<String>,
    pub iro_contact_l_name: Option<String>,
    pub iro_email: Option<String>,
    pub iro_license_expiration: Option<Date>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::eligible_case::Entity")]
    EligibleCase,
}

impl Related<super::eligible_case::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EligibleCase.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

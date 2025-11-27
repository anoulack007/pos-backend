use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole{
    #[sea_orm(string_value = "ADMIN")]
    Admin,
    #[sea_orm(string_value = "USER")]
    Cashier,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username:String,
    pub password_hash:String,
    pub full_name: Option<String>,
    pub role: UserRole,
    pub is_active:bool,
    pub created_at: DateTime,
    // pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    
}

impl ActiveModelBehavior for ActiveModel{}
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateUser, fields(username, email, password, avatar_url, about), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[pick(arg(ident = UpdateUser, fields(id, username, email, password, avatar_url, about), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[sea_orm::model]
#[derive(DeriveEntityModel, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub about: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(has_many)]
    pub anime_list: HasMany<super::anime_list::Entity>,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(has_many)]
    pub reviews: HasMany<super::reviews::Entity>,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(has_many)]
    pub favorites: HasMany<super::favorites::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

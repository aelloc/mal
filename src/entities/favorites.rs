use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::*;

#[derive(Pick, Omit)]
#[pick(arg(ident = CreateFavorite, fields(user_id, anime_id), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[pick(arg(ident = UpdateFavorite, fields(user_id, anime_id), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, JsonSchema)]
#[sea_orm(table_name = "favorites")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub anime_id: i64,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<super::users::Entity>,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<super::animes::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

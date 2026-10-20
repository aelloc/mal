use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateAnimeStudio, fields(anime_id, studio_id), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[pick(arg(ident = UpdateAnimeStudio, fields(anime_id, studio_id), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, JsonSchema)]
#[sea_orm(table_name = "anime_studios")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub anime_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub studio_id: i64,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<super::animes::Entity>,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "studio_id", to = "id")]
    pub studio: BelongsTo<super::studios::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

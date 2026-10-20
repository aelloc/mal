use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;
#[derive(Pick)]
#[pick(arg(ident = CreateAnimeGenre, fields(anime_id, genre_id), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[pick(arg(ident = UpdateAnimeGenre, fields(anime_id, genre_id), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, JsonSchema)]
#[sea_orm(table_name = "anime_genres")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub anime_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub genre_id: i64,

    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<super::animes::Entity>,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "genre_id", to = "id")]
    pub genre: BelongsTo<super::genres::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateGenre, fields(name), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[pick(arg(ident = UpdateGenre, fields(id, name), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[sea_orm::model]
#[derive(DeriveEntityModel, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[sea_orm(table_name = "genres")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(has_many, via = "anime_genres")]
    pub animes: HasMany<super::animes::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

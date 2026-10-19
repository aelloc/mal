use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateAnime, fields(title, title_english, title_japanese, r#type, synopsis, episodes, status, aired_from, aired_to, season, year, source, duration_min, rating, image_url), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel), forward_attrs()))]
#[pick(arg(ident = UpdateAnime, fields(title, title_english, title_japanese, r#type, synopsis, episodes, status, aired_from, aired_to, season, year, source, duration_min, rating, image_url), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel), forward_attrs()))]
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, JsonSchema)]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_english: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_japanese: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[sea_orm(column_type = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synopsis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aired_from: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aired_to: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

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
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(has_many, via = "anime_genres")]
    pub genres: HasMany<super::genres::Entity>,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(has_many, via = "anime_studios")]
    pub studios: HasMany<super::studios::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

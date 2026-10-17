use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateAnime, fields(title, title_english, title_japanese, r#type, synopsis, episodes, status, aired_from, aired_to, season, year, source, duration_min, rating, image_url), derive(Debug, Serialize, Deserialize, JsonSchema), forward_attrs()))]
#[pick(arg(ident = UpdateAnime, fields(title, title_english, title_japanese, r#type, synopsis, episodes, status, aired_from, aired_to, season, year, source, duration_min, rating, image_url), derive(Debug, Serialize, Deserialize, JsonSchema), forward_attrs()))]
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, JsonSchema)]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub r#type: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub synopsis: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub aired_from: Option<chrono::NaiveDate>,
    pub aired_to: Option<chrono::NaiveDate>,
    pub season: Option<String>,
    pub year: Option<i32>,
    pub source: Option<String>,
    pub duration_min: Option<i32>,
    pub rating: Option<String>,
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

use schemars::JsonSchema;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use sea_orm_migration::async_trait::async_trait;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateAnimeList, fields(user_id, anime_id,status, score, episodes_watched, start_date,finish_date,rewatching,times_rewatched,tags,comments), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[pick(arg(ident = UpdateAnimeList, fields(id,user_id, anime_id,status, score, episodes_watched, start_date,finish_date,rewatching,times_rewatched,tags,comments),derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, JsonSchema)]
#[sea_orm(table_name = "anime_list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique_key = "user_anime")]
    pub user_id: i64,
    #[sea_orm(unique_key = "user_anime")]
    pub anime_id: i64,
    pub status: Option<String>,
    pub score: Option<i32>,
    pub episodes_watched: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub finish_date: Option<chrono::NaiveDate>,
    pub rewatching: bool,
    pub times_rewatched: i32,
    pub tags: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub comments: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: chrono::NaiveDateTime,

    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<super::users::Entity>,
    #[serde(skip)]
    #[schemars(skip)]
    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<super::animes::Entity>,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(self)
    }
}

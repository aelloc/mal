use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateReview, fields(user_id, anime_id, score, body), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[pick(arg(ident = UpdateReview, fields(id, user_id, anime_id, score, body), derive(Debug, Serialize, Deserialize, JsonSchema, DeriveIntoActiveModel),forward_attrs()))]
#[sea_orm::model]
#[derive(DeriveEntityModel, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[sea_orm(table_name = "reviews")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub anime_id: i64,
    pub score: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub body: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::NaiveDateTime,
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

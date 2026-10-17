use crate::entities::animes::{ActiveModel, Model};
use aide::{
    axum::{ApiRouter, routing::post_with},
    transform::TransformOperation,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{
    State as AppState,
    api::{AppResponse, extractor::Authorized},
    entities::animes::CreateAnime,
};

pub fn router(state: AppState) -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_anime, create_anime_docs).get_with(list_anime, list_animes_docs),
        )
        .with_state(state)
}

#[tracing::instrument]
pub async fn create_anime(
    State(state): State<AppState>,
    // Authorized(user): Authorized,
    Json(request): Json<CreateAnime>,
) -> AppResponse<Model> {
    // println!("User: {}", user);

    let anime = ActiveModel {
        title: Set(request.title),
        title_english: Set(request.title_english),
        title_japanese: Set(request.title_japanese),
        r#type: Set(request.r#type),
        synopsis: Set(request.synopsis),
        episodes: Set(request.episodes),
        status: Set(request.status),
        aired_from: Set(request.aired_from),
        aired_to: Set(request.aired_to),
        season: Set(request.season),
        year: Set(request.year),
        source: Set(request.source),
        duration_min: Set(request.duration_min),
        rating: Set(request.rating),
        image_url: Set(request.image_url),
        ..Default::default()
    };

    match anime.insert(&state.db_conn).await {
        Ok(anime) => AppResponse {
            status_code: StatusCode::OK.into(),
            message: "anime created successfully".into(),
            data: Some(anime),
        },
        Err(err) => AppResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: format!("Error occured while creating anime: {:?}", err),
            data: None,
        },
    }
}

pub fn create_anime_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

pub async fn list_anime(Authorized(_user): Authorized) -> AppResponse<Vec<Model>> {
    AppResponse {
        status_code: StatusCode::OK.into(),
        message: String::new(),
        data: None,
    }
}
pub fn list_animes_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

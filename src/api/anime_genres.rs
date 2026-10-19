use aide::{
    axum::{ApiRouter, routing::post_with},
    transform::TransformOperation,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, IntoActiveModel};

use crate::{
    State as AppState,
    api::AppResponse,
    entities::anime_genres::{CreateAnimeGenre, Model},
};

pub fn router(state: AppState) -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_anime_genres, create_anime_genres_docs),
        )
        .with_state(state)
}

#[tracing::instrument]
pub async fn create_anime_genres(
    // Authorized(_user): Authorized,
    state: State<AppState>,
    Json(request): Json<CreateAnimeGenre>,
) -> AppResponse<Model> {
    let result = Model {
        anime_id: request.anime_id,
        genre_id: request.genre_id,
    }
    .into_active_model()
    .insert(&state.db_conn)
    .await;

    match result {
        Ok(genre) => AppResponse {
            status_code: StatusCode::CREATED.as_u16(),
            message: String::from("anime genre successfully created"),
            data: Some(genre),
        },
        Err(err) => {
            tracing::error!("create anime genre database error  {err}");
            return AppResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: String::from("failed while insert data to database"),
                data: None,
            };
        }
    }
}

pub fn create_anime_genres_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime Genres")
}

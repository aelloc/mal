use crate::{
    State as AppState,
    api::{AppError, AppResponse, consruct_response},
    entities::genres::{CreateGenre, Entity as GenreEntity, Model, UpdateGenre},
};
use aide::{
    axum::{
        ApiRouter,
        routing::{get_with, post_with},
    },
    transform::TransformOperation,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};

pub fn router(state: AppState) -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_genre, create_genre_docs).get_with(list_genre, list_genre_docs),
        )
        .api_route(
            "/{id}",
            get_with(get_genre, get_genre_docs)
                .put_with(update_genre, update_genre_docs)
                .delete_with(delete_genre, delete_genre_docs),
        )
        .with_state(state)
}

#[tracing::instrument]
pub async fn create_genre(
    State(state): State<AppState>,
    Json(request): Json<CreateGenre>,
) -> Result<AppResponse<Model>, AppError> {
    let anime = request.into_active_model().insert(&state.db_conn).await;

    consruct_response(
        anime,
        "anime created successfully".into(),
        "couldn't create anime".into(),
    )
}

pub fn create_genre_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn list_genre(
    State(state): State<AppState>,
) -> Result<AppResponse<Vec<Model>>, AppError> {
    let animes = GenreEntity::find().all(&state.db_conn).await;

    consruct_response(
        animes,
        "get all anime".into(),
        "couldn't get all anime".into(),
    )
}
pub fn list_genre_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn get_genre(
    Path(anime_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<AppResponse<Model>, AppError> {
    // FIXME: remove unwrap
    let anime = GenreEntity::find_by_id(anime_id)
        .one(&state.db_conn)
        .await
        .and_then(|l| Ok(l.unwrap()));

    consruct_response(
        anime,
        "get all anime".into(),
        "couldn't get all anime".into(),
    )
}

pub fn get_genre_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn update_genre(
    Path(anime_id): Path<i64>,
    State(state): State<AppState>,
    Json(request): Json<UpdateGenre>,
) -> Result<AppResponse<Model>, AppError> {
    let mut anime = request.into_active_model();
    anime.id = Set(anime_id);
    let anime = anime.update(&state.db_conn).await;

    consruct_response(
        anime,
        "anime updated successfully".into(),
        "error occured while updating anime".into(),
    )
}

pub fn update_genre_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn delete_genre(
    Path(anime_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<AppResponse<Model>, AppError> {
    // FIXME: remove unwrap
    match GenreEntity::delete_by_id(anime_id)
        .exec(&state.db_conn)
        .await
    {
        Ok(_) => Ok(AppResponse {
            status_code: StatusCode::OK.into(),
            message: "anime deleted successfully".into(),
            data: None,
        }),
        Err(err) => Err(AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: format!("error occured while deleting anime: {:?}", err),
        }),
    }
}

pub fn delete_genre_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

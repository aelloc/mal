use crate::api::consruct_response;
use crate::entities::animes::UpdateAnime;
use crate::{State as AppState, api::AppResponse, entities::animes::CreateAnime};
use crate::{
    api::AppError,
    entities::animes::{Entity as AnimeEntity, Model},
};
use aide::axum::routing::get_with;
use aide::{
    axum::{ApiRouter, routing::post_with},
    transform::TransformOperation,
};
use axum::extract::Path;
use axum::http::request;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::IntoActiveModel;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

pub fn router(state: AppState) -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_anime, create_anime_docs).get_with(list_anime, list_anime_docs),
        )
        .api_route(
            "/{id}",
            get_with(get_anime, get_animes_docs)
                .put_with(update_anime, update_anime_docs)
                .delete_with(delete_anime, delete_anime_docs),
        )
        .with_state(state)
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn create_anime(
    State(state): State<AppState>,
    Json(request): Json<CreateAnime>,
) -> Result<AppResponse<Model>, AppError> {
    let anime = request.into_active_model().insert(&state.db_conn).await;

    consruct_response(
        anime,
        "anime created successfully".into(),
        "couldn't create anime".into(),
    )
}

pub fn create_anime_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn list_anime(
    State(state): State<AppState>,
) -> Result<AppResponse<Vec<Model>>, AppError> {
    let animes = AnimeEntity::find().all(&state.db_conn).await;

    consruct_response(
        animes,
        "get all anime".into(),
        "couldn't get all anime".into(),
    )
}
pub fn list_anime_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn get_anime(
    Path(anime_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<AppResponse<Model>, AppError> {
    let anime = AnimeEntity::find_by_id(anime_id)
        .one(&state.db_conn)
        .await
        .and_then(|l| Ok(l.unwrap()));

    consruct_response(
        anime,
        "get all anime".into(),
        "couldn't get all anime".into(),
    )
}

pub fn get_animes_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn update_anime(
    Path(anime_id): Path<i64>,
    State(state): State<AppState>,
    Json(request): Json<UpdateAnime>,
) -> Result<AppResponse<Model>, AppError> {
    let mut anime = request.into_active_model();
    anime.id = Set(anime_id);

    match anime.update(&state.db_conn).await {
        Ok(anime) => Ok(AppResponse {
            status_code: StatusCode::OK.into(),
            message: "anime updated successfully".into(),
            data: Some(anime),
        }),
        Err(err) => Err(AppError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: format!("Error occured while updating anime: {:?}", err),
        }),
    }
}

pub fn update_anime_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[axum::debug_handler]
#[tracing::instrument]
pub async fn delete_anime(
    Path(anime_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<AppResponse<Model>, AppError> {
    let entity = AnimeEntity::find_by_id(anime_id)
        .one(&state.db_conn)
        .await
        .unwrap();

    Ok(AppResponse {
        status_code: StatusCode::OK.into(),
        message: String::new(),
        data: entity,
    })
}

pub fn delete_anime_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

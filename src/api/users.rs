use crate::{
    State as AppState,
    api::{AppError, AppResponse, consruct_response},
    entities::users::{CreateUser, Entity as UserEntity, Model, UpdateUser},
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
        .api_route("/register", post_with(create_user, create_user_docs))
        .api_route("/login", post_with(login_user, login_user_docs))
        .api_route("/", get_with(list_user, list_user_docs))
        .api_route(
            "/{id}",
            get_with(get_user, get_user_docs)
                .put_with(update_user, update_user_docs)
                .delete_with(delete_user, delete_user_docs),
        )
        .with_state(state)
}

#[tracing::instrument]
pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUser>,
) -> Result<AppResponse<Model>, AppError> {
    let anime = request.into_active_model().insert(&state.db_conn).await;

    consruct_response(
        anime,
        "anime created successfully".into(),
        "couldn't create anime".into(),
    )
}

pub fn create_user_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn login_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUser>,
) -> Result<AppResponse<Model>, AppError> {
    let user_exists = UserEntity::find_by_email(request.email)
        .one(&state.db_conn)
        .await
        .and_then(|record| record.ok_or(sea_orm::DbErr::RecordNotFound("Not found".into())));

    let _ = dbg!(user_exists);

    Ok(AppResponse {
        status_code: StatusCode::OK.into(),
        message: "".into(),
        data: None,
    })
}

pub fn login_user_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn list_user(State(state): State<AppState>) -> Result<AppResponse<Vec<Model>>, AppError> {
    let animes = UserEntity::find().all(&state.db_conn).await;

    consruct_response(
        animes,
        "get all anime".into(),
        "couldn't get all anime".into(),
    )
}
pub fn list_user_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn get_user(
    Path(anime_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<AppResponse<Model>, AppError> {
    // FIXME: remove unwrap
    let anime = UserEntity::find_by_id(anime_id)
        .one(&state.db_conn)
        .await
        .and_then(|l| Ok(l.unwrap()));

    consruct_response(
        anime,
        "get all anime".into(),
        "couldn't get all anime".into(),
    )
}

pub fn get_user_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn update_user(
    Path(anime_id): Path<i64>,
    State(state): State<AppState>,
    Json(request): Json<UpdateUser>,
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

pub fn update_user_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

#[tracing::instrument]
pub async fn delete_user(
    Path(anime_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<AppResponse<Model>, AppError> {
    // FIXME: remove unwrap
    match UserEntity::delete_by_id(anime_id)
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

pub fn delete_user_docs(op: TransformOperation) -> TransformOperation {
    op.tag("Anime")
}

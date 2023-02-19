use axum::{Router, Json};
use axum::routing::get;
use axum::extract::{Path, Extension};
use axum::http::StatusCode;

use crate::http::ApiContext;
use crate::database::crud::digital_timer::DigitalTimerCRUD;
use crate::database::models::digital_timer::DigitalTimer;
use crate::errors::Error;

pub fn router() -> Router {
    Router::new()
        .route("/api/digital_timer/:id", get(get_timer).delete(delete_timer))
        .route(
            "/api/digital_timer",
            get(list_timers).post(create_timer),
        )
}

async fn get_timer(
    ctx: Extension<ApiContext>,
    Path(id): Path<i64>
) -> Json<DigitalTimer> {
    let timer = DigitalTimerCRUD::get_one(&ctx.pool, id).await.unwrap();
    Json(timer.into())
}

async fn delete_timer(
    ctx: Extension<ApiContext>,
    Path(id): Path<i64>
) -> Result<StatusCode, Error> {
    DigitalTimerCRUD::delete_one(&ctx.pool, id).await?; 
    Ok(StatusCode::OK)
}

async fn list_timers(
    ctx: Extension<ApiContext>,
) -> Result<Json<Vec<DigitalTimer>>, Error> {
    let timers = DigitalTimerCRUD::get_all(&ctx.pool).await?;
    Ok(Json(
        timers
            .into_iter()
            .map(|timer| timer.into())
            .collect()
    ))
}

async fn create_timer(
    ctx: Extension<ApiContext>,
    Json(timer): Json<DigitalTimer>,
) -> Result<Json<DigitalTimer>, Error> {
    let id = DigitalTimerCRUD::insert_one(&ctx.pool, &timer).await?;
    let mut timer = timer;
    timer.id = Some(id);
    Ok(Json(timer))
}
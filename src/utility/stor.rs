use serde::Serialize;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

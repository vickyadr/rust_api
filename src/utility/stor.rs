use std::collections::HashMap;

use serde::Serialize;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
}
#[derive(Serialize)]
pub struct NoDataResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<i8>,
}

#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Vec<T>,
}

#[derive(Serialize)]
pub struct KeyValResponse<K, V> {
    pub success: bool,
    pub message: String,
    pub data: HashMap<K, V>,
}

impl NoDataResponse {
    pub fn new(msg: String, scc: bool) -> NoDataResponse {
        NoDataResponse {
            success: scc,
            message: msg,
            data: Vec::new(),
        }
    }
}

impl<T> GenericResponse<T> {
    pub fn new(dt: Vec<T>, msg: String, scc: bool) -> GenericResponse<T> {
        GenericResponse::<T> {
            success: scc,
            message: msg,
            data: dt,
        }
    }
}

impl<K, V> KeyValResponse<K, V> {
    pub fn new(dt: HashMap<K, V>, msg: String, scc: bool) -> KeyValResponse<K, V> {
        KeyValResponse::<K, V> {
            success: scc,
            message: msg,
            data: dt,
        }
    }
}

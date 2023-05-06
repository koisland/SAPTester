use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    battle::response::post_battle,
    db::response::{get_food, get_pet},
};

pub fn db_routes() -> Router {
    Router::new()
        .route("/db/pets", get(get_pet))
        .route("/db/foods", get(get_food))
}

pub fn battle_routes() -> Router {
    Router::new().route("/battle", post(post_battle))
}

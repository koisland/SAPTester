use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use itertools::Itertools;
use saptest::{
    db::record::{FoodRecord, PetRecord},
    Entity, SAPQuery, SAPDB,
};
use std::collections::HashMap;

#[derive(Debug)]
struct APIQuery {
    qtype: Entity,
    params: HashMap<String, String>,
}

impl From<APIQuery> for SAPQuery {
    fn from(query: APIQuery) -> Self {
        let mut db_query = SAPQuery::from_iter(
            query
                .params
                .into_iter()
                .map(|(param, param_vals)| (param, vec![param_vals])),
        );
        db_query.set_table(query.qtype);
        db_query
    }
}

pub async fn get_pet(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let query = APIQuery {
        qtype: Entity::Pet,
        params,
    }
    .into();

    SAPDB
        .execute_query(query)
        .map(|records| {
            (
                StatusCode::FOUND,
                Json(
                    records
                        .into_iter()
                        .filter_map(|rec| PetRecord::try_from(rec).ok())
                        .collect_vec(),
                ),
            )
        })
        .map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn get_food(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let query = APIQuery {
        qtype: Entity::Food,
        params,
    }
    .into();

    SAPDB
        .execute_query(query)
        .map(|records| {
            (
                StatusCode::FOUND,
                Json(
                    records
                        .into_iter()
                        .filter_map(|rec| FoodRecord::try_from(rec).ok())
                        .collect_vec(),
                ),
            )
        })
        .map_err(|_| StatusCode::BAD_REQUEST)
}

#[cfg(test)]
mod tests {
    use crate::app;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_food_w_params() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/db/foods?name=Apple&pack=Turtle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::FOUND);

        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let foods = body.as_array().unwrap();
        assert_eq!(foods.len(), 1);

        let food_name = foods[0].get("name").and_then(|name| name.as_str());
        let food_pack = foods[0].get("pack").and_then(|name| name.as_str());
        assert!(food_name == Some("Apple") && food_pack == Some("Turtle"));
    }

    #[tokio::test]
    async fn test_get_all_foods() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/db/foods")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::FOUND);
    }

    #[tokio::test]
    async fn test_get_pets_w_params() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/db/pets?effect_trigger=Faint&pack=Turtle&lvl=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::FOUND);

        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let pets = body.as_array().unwrap();

        // There are 9 pets in the turtle pack that have faint triggers.
        assert_eq!(pets.len(), 9);
    }

    #[tokio::test]
    async fn test_get_all_pets() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/db/pets")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::FOUND);
    }
}

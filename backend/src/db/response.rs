use axum::headers::AccessControlAllowOrigin;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json, TypedHeader};
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
                TypedHeader(AccessControlAllowOrigin::ANY),
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
                TypedHeader(AccessControlAllowOrigin::ANY),
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

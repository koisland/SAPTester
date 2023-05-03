use axum::{http::StatusCode, response::IntoResponse, Json};
use saptest::{error::SAPTestError, Team};

use super::team::Teams;

pub async fn battle(Json(teams): Json<Teams>) -> impl IntoResponse {
    let friend_team: Result<Team, SAPTestError> = teams.friend_team.try_into();
    let enemy_team: Result<Team, SAPTestError> = teams.enemy_team.try_into();

    if let Err(err) = friend_team {
        let err_msg = format!("Invalid Friend Team: {:?}", err);
        (StatusCode::NOT_ACCEPTABLE, err_msg)
    } else if let Err(err) = enemy_team {
        let err_msg = format!("Invalid Enemy Team: {:?}", err);
        (StatusCode::NOT_ACCEPTABLE, err_msg)
    } else {
        (StatusCode::ACCEPTED, "OK".to_string())
    }
}

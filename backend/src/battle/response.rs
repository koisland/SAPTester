use axum::{http::StatusCode, Json};
use saptest::{
    create_battle_digraph, error::SAPTestError, teams::team::TeamFightOutcome, Team, TeamCombat,
};
use serde::{Deserialize, Serialize};

use super::{team::Teams, ALLOWED_NUM_TURNS};

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleResponse {
    status: Option<String>,
    outcome: TeamFightOutcome,
    friend_team: Option<Team>,
    enemy_team: Option<Team>,
    num_turns: usize,
    digraph: Option<String>,
}

impl Default for BattleResponse {
    fn default() -> Self {
        Self {
            status: Default::default(),
            outcome: TeamFightOutcome::None,
            friend_team: Default::default(),
            enemy_team: Default::default(),
            num_turns: Default::default(),
            digraph: Default::default(),
        }
    }
}

pub async fn post_battle(Json(teams): Json<Teams>) -> Json<BattleResponse> {
    let mut resp = BattleResponse::default();
    let friend_team: Result<Team, SAPTestError> = teams.friend_team.try_into();
    let enemy_team: Result<Team, SAPTestError> = teams.enemy_team.try_into();

    let Ok(mut team) = friend_team else {
        let err_msg = format!("Invalid Friend Team: {:?}", friend_team.unwrap_err());
        resp.status = Some(err_msg);
        return Json(resp)
    };
    let Ok(mut enemy_team) = enemy_team else {
        let err_msg = format!("Invalid Enemy Team: {:?}", enemy_team.unwrap_err());
        resp.status = Some(err_msg);
        return Json(resp)
    };

    let mut num_turns = 0;
    let mut outcome = Ok(TeamFightOutcome::None);
    while let Ok(TeamFightOutcome::None) = outcome {
        if num_turns > ALLOWED_NUM_TURNS {
            outcome = Err(SAPTestError::InvalidTeamAction {
                subject: "Battle Duration".to_owned(),
                reason: format!("Reached maximum turn limit, {num_turns}"),
            });
            break;
        }
        outcome = team.fight(&mut enemy_team);
        num_turns += 1;
    }

    let digraph = create_battle_digraph(&team, false);
    resp.friend_team = Some(team);
    resp.enemy_team = Some(enemy_team);
    resp.digraph = Some(digraph);
    resp.num_turns = num_turns;

    if let Ok(battle_outcome) = outcome {
        resp.outcome = battle_outcome;
        resp.status = Some(StatusCode::ACCEPTED.to_string());
    } else {
        resp.status = Some(outcome.unwrap_err().to_string());
    }

    Json(resp)
}

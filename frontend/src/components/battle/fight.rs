use crate::records::query::post_battle;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ui::BattleUIState;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BattleResponse {
    status: Option<String>,
    outcome: String,
    friend_team: Option<Value>,
    enemy_team: Option<Value>,
    num_turns: usize,
    digraph: Option<String>,
}

pub fn FightSummaryModal<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    outcome: &UseRef<Option<BattleResponse>>,
    modal_state: &'a UseState<&str>,
) -> Element<'a> {
    outcome.with(|outcome| println!("{outcome:?}"));
    // let outcome_summary = outcome.with(|outcome| {
    //     let winner = match outcome.outcome {
    //         TeamFightOutcome::Win => "Friend Team won!",
    //         TeamFightOutcome::Loss => "Enemy Team won!",
    //         TeamFightOutcome::Draw => "None (Draw)",
    //         TeamFightOutcome::None => "None (Incomplete",
    //     };
    //     let friend_team_div = outcome.friend_team.as_ref().and_then(|team| {
    //         cx.render(rsx! {
    //             div {
    //                 class: "w3-container w3-leftbar",
    //                 "{team}"
    //             }
    //         })
    //     });
    //     let enemy_team_div = outcome.enemy_team.as_ref().and_then(|team| {
    //         cx.render(rsx! {
    //             div {
    //                 class: "w3-container w3-leftbar",
    //                 "{team}"
    //             }
    //         })
    //     });
    //     cx.render(rsx! {
    //         h2 {
    //             "{winner}"
    //         }
    //         friend_team_div
    //         enemy_team_div
    //     })
    // });

    cx.render(rsx! {
        div { class: "w3-container w3-modal", display: "{modal_state.get()}",
            div { class: "w3-container w3-modal-content",
                header { class: "w3-container w3-black",
                    span {
                        onclick: move |_| modal_state.set("none"),
                        class: "w3-button w3-display-topright",
                        "X"
                    }
                    "Outcome"
                }
            }
        }
    })
}

pub fn FightSummary<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let summary_state = use_state(cx, || "none");
    let post_battle_outcome: &UseRef<Option<BattleResponse>> = use_ref(cx, || None);

    cx.render(rsx! {
        div { class: "w3-container w3-xlarge",
            button {
                class: "w3-button w3-block w3-red",
                onclick: move |_| {
                    cx.spawn({
                        let post_battle_outcome = post_battle_outcome.to_owned();
                        let teams = cx.props.teams.with(|teams| teams.to_owned());
                        async move {
                            let res = post_battle(teams).await;
                            post_battle_outcome.set(res.ok())
                        }
                    });
                    summary_state.set("block")
                },
                "Fight!"
            }
            FightSummaryModal(cx, post_battle_outcome, summary_state)
        }
    })
}

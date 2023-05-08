use std::error::Error;

use dioxus::prelude::*;
use itertools::Itertools;

use super::ui::BattleUIState;


pub fn FightSummaryModal<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    outcome: &UseRef<PostBattleState>,
    modal_state: &'a UseState<&str>,
) -> Element<'a> {
    let outcome_summary = outcome.with(|outcome| {
        let winner = match outcome.outcome {
            TeamFightOutcome::Win => "Friend Team won!",
            TeamFightOutcome::Loss => "Enemy Team won!",
            TeamFightOutcome::Draw => "None (Draw)",
            TeamFightOutcome::None => "None (Incomplete",
        };
        let friend_team_div = outcome.friend_team.as_ref().and_then(|team| {
            cx.render(rsx! {
                div {
                    class: "w3-container w3-leftbar",
                    "{team}"
                }
            })
        });
        let enemy_team_div = outcome.enemy_team.as_ref().and_then(|team| {
            cx.render(rsx! {
                div {
                    class: "w3-container w3-leftbar",
                    "{team}"
                }
            })
        });
        cx.render(rsx! {
            h2 {
                "{winner}"
            }
            friend_team_div
            enemy_team_div
        })
    });

    cx.render(rsx! {
        div {
            class: "w3-container w3-modal",
            display: "{modal_state.get()}",
            div {
                class: "w3-container w3-modal-content",
                header {
                    class: "w3-container w3-black",
                    span {
                        onclick: move |_| modal_state.set("none"),
                        class: "w3-button w3-display-topright",
                        "X"
                    }
                    "Outcome"
                }
                outcome_summary
            }
        }
    })
}

pub fn FightSummary<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let summary_state = use_state(&cx, || "none");
    let post_battle_outcome = use_ref(&cx, PostBattleState::default);
    cx.render(rsx! {
        div {
            class: "w3-container w3-xlarge",
            button {
                class: "w3-button w3-block w3-red",
                onclick: move |_| {
                    let outcome = setup_team_fight(cx).unwrap_or_else(|_| PostBattleState::default());
                    post_battle_outcome.set(outcome);
                    summary_state.set("block")
                },
                "Fight!"
            }
            FightSummaryModal(cx, post_battle_outcome, summary_state)
        }
    })
}

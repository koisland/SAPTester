use crate::records::query::post_battle;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ui::BattleUIState;

const QUICK_CHART_GRAPHVIZ_APIURL: &str = "https://quickchart.io/graphviz?graph=";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BattleResponse {
    pub status: Option<String>,
    pub outcome: String,
    pub friend_team: Option<Value>,
    pub enemy_team: Option<Value>,
    pub num_turns: usize,
    pub digraph: Option<String>,
}

pub fn PetValueRow<'a>(cx: Scope<'a, BattleUIState<'a>>, pets: &[Value]) -> Element<'a> {
    cx.render(rsx! {
        tr {
            pets.iter().map(|pet| {
                let name = pet.get("name").and_then(|name| name.as_str());
                cx.render(rsx! {
                    td { name }
                })
            })
        }
    })
}

pub fn PostBattleTeamContainer<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    team: Option<&Value>,
) -> Element<'a> {
    team.and_then(|team| {
        let fainted_pet_elems = team
            .get("fainted")
            .and_then(|pets| pets.as_array())
            .and_then(|pets| PetValueRow(cx, pets));

        let pet_elems = team
            .get("friends")
            .and_then(|pets| pets.as_array())
            .and_then(|pets| PetValueRow(cx, pets));

        cx.render(rsx! {
            table { class: "w3-table w3-responsive",
                h4 { "Alive" }
                pet_elems,
                h4 { "Fainted" }
                fainted_pet_elems
            }
        })
    })
}

pub fn FightSummaryModal<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    outcome: &UseRef<Option<BattleResponse>>,
    modal_state: &'a UseState<&str>,
) -> Element<'a> {
    let digraph_code_state = use_state(cx, || "block");
    let outcome_summary = outcome.with(|outcome| {
        let Some(outcome) = outcome else {
            return None
        };
        let (mut friend_title, mut enemy_title) = (String::from("Friend"), String::from("Enemy"));
        let mut is_undecided = false;

        match &outcome.outcome[..] {
            "Win" => friend_title.push_str(" (Winner)"),
            "Loss" => enemy_title.push_str(" (Winner)"),
            "None" => is_undecided = true,
            _ => {}
        };

        // Format of outcome:
        // {"fainted":[],"friends":[],"max_size":5,"name":"Enemy","seed": usize,"sold":[],"stored_friends":[],"triggers":[]}
        let friend_team_div = PostBattleTeamContainer(cx, outcome.friend_team.as_ref());
        let enemy_team_div = PostBattleTeamContainer(cx, outcome.enemy_team.as_ref());

        cx.render(rsx! {
            // If it turn limit and battle unfinished, show message.
            is_undecided.then(|| cx.render(rsx! {
                "Unfinished battle. Reached turn limit of 250."
            })),

            h3 { class: "w3-panel w3-card w3-light-grey", friend_title }
            friend_team_div,
            h3 { class: "w3-panel w3-card w3-light-grey", enemy_title }
            enemy_team_div,
            br {}

            outcome.digraph.as_ref().and_then(|digraph_str| {
                let graphvis_chart_request = format!("{QUICK_CHART_GRAPHVIZ_APIURL}{digraph_str}");
                cx.render(rsx! {
                    div { class: "w3-panel w3-card w3-light-grey",
                        h3 { "Graph" },
                        h6 {
                            "Produced with "
                            a { href: "https://quickchart.io/documentation/", "QuickChart" }
                            "."
                        }
                        div { class: "w3-container",
                            h4 { class: "w3-panel w3-card w3-pale-green",
                                onclick: |_| {
                                    if *digraph_code_state.get() == "none" {
                                        digraph_code_state.set("block")
                                    } else {
                                        digraph_code_state.set("none")
                                    }
                                },
                                "DOT"
                            }
                            div { class: "w3-code", display: "{digraph_code_state.get()}",
                                "{digraph_str}",
                            }
                        }
                        div { class: "w3-container",
                            h4 { class: "w3-panel w3-card w3-pale-green",
                                "Graphviz"
                            }
                            img { class: "w3-image",
                                src: "{graphvis_chart_request}"
                            }
                        }

                        br {}
                        br {}
                    }
                })
            }),

            br {}
        })
    });

    cx.render(rsx! {
        div { class: "w3-container w3-modal", display: "{modal_state.get()}",
            div { class: "w3-display-container",
                h2 { class: "w3-container w3-black", "Outcome" }
                button {
                    class: "w3-button w3-red w3-display-topright",
                    onclick: move |_| modal_state.set("none"),
                    "X"
                }
            }
            div { class: "w3-container",
                div { class: "w3-container w3-white", outcome_summary }
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

use dioxus::prelude::*;
// use itertools::Itertools;
// use saptest::Team;

use super::ui::BattleUIState;

// pub fn setup_team_fight<'a>(cx: Scope<'a, BattleUIState<'a>>) {
//     cx.props.teams.with(|teams| {
//         let friends = teams.get("Friend");
//         let enemies = teams.get("Enemy");

//         if let (Some(friends), Some(enemies)) = (friends, enemies) {
//             let friend_team = friends
//                 .iter()
//                 .rev()
//                 .map(|(_, pet)| pet)
//                 .cloned()
//                 .collect_vec();
//             let enemy_team = enemies
//                 .iter()
//                 .rev()
//                 .map(|(_, pet)| pet)
//                 .cloned()
//                 .collect_vec();
//         }
//     })
// }

pub fn FightSummary<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element {
    let summary_state = use_state(cx, || "none");

    cx.render(rsx! {
        div {
            class: "w3-container w3-xlarge",
            button {
                class: "w3-button w3-block w3-red",
                onclick: move |_| {
                    // setup_team_fight(cx);
                    summary_state.set("block")
                },
                "Fight!"
            }
        }
        div {
            class: "w3-modal",
            display: "{summary_state.get()}",
            div {
                class: "w3-modal-content",
                header {
                    class: "w3-container w3-black",
                    span {
                        onclick: move |_| summary_state.set("none"),
                        class: "w3-button w3-display-topright",
                        "X"
                    }
                    "Outcome"
                }
                div {
                    class: "w3-container",
                    "Winner was _."
                }


            }
        }
    })
}

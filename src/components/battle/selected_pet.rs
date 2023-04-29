use dioxus::prelude::*;
use log::info;
use saptest::{
    pets::pet::{MAX_PET_LEVEL, MAX_PET_STATS, MIN_PET_LEVEL, MIN_PET_STATS},
    Effect, Food, Statistics,
};

use crate::{
    components::battle::{
        ui::BattleUIState,
        utils::{assign_pet_level, assign_pet_stats, get_selected_pet},
    },
    utils::extract_urls::{ATTACK_ICON, HEALTH_ICON},
};

fn LabeledStatInput<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    stat_label: &'a str,
    starting_value: isize,
    pet_stats: Statistics,
) -> Element<'a> {
    let is_valid_state = use_state(cx, || true);
    cx.render(rsx! {
        div {
            class: "w3-container",
            img {
                class: "w3-image w3-center",
                width: "10%",
                height: "10%",
                title: "{stat_label}",
                src: if stat_label == "Health" {HEALTH_ICON} else {ATTACK_ICON},
            }
            input {
                class: if **is_valid_state { "w3-input w3-center w3-half"} else { "w3-input w3-half w3-center w3-pale-red"},
                "type": "number",
                placeholder: "{stat_label}",
                value: "{starting_value}",
                min: "{MIN_PET_STATS}",
                max: "{MAX_PET_STATS}",
                required: true,
                onchange: move |evt| {
                    if let Ok(input_stat_value) = &evt.data.value.parse::<isize>().map(|value| value.clamp(MIN_PET_STATS, MAX_PET_STATS)) {
                        is_valid_state.set(true);
                        // Assign pet stats.
                        let stats = Statistics {
                            attack:  if stat_label == "Attack" { *input_stat_value } else { pet_stats.attack },
                            health:  if stat_label == "Health" { *input_stat_value } else { pet_stats.health }
                        };
                        if let Err(err) = assign_pet_stats(cx, stats) {
                            info!("{err}")
                        }
                    } else {
                        is_valid_state.set(false)
                    }
                }
            }
        }
    })
}

fn PetStatContainer<'a>(cx: Scope<'a, BattleUIState<'a>>, pet_stats: Statistics) -> Element<'a> {
    cx.render(rsx! {
        form {
            class: "w3-container",
            LabeledStatInput(cx, "Attack", pet_stats.attack, pet_stats),
            LabeledStatInput(cx, "Health", pet_stats.health, pet_stats)
        }
    })
}

fn PetEffectContainer<'a>(cx: Scope<'a, BattleUIState<'a>>, pet_effects: &[Effect]) -> Element<'a> {
    let Some(selected_pet) = get_selected_pet(cx) else {
        return cx.render(rsx! {
            div {
                class: "w3-container",
                "No Selected Pet"
            }
        })
    };

    cx.render(rsx! {
        div {
            class: "w3-container",
            h2 { "Effect" }
            // Allow level selection.
            select {
                class: "w3-select w3-center",
                value: "{selected_pet.get_level()}",
                (MIN_PET_LEVEL..=MAX_PET_LEVEL).map(|lvl| {
                    rsx! {
                        option {
                            value: "{lvl}",
                            onclick: move |_| {
                                if let Err(err) = assign_pet_level(cx, lvl) {
                                    info!("{err}")
                                }
                            },
                            "{lvl}"
                        }
                    }
                })
            }
            for effect in pet_effects.iter() {
                div {
                    class: "w3-panel w3-leftbar",
                    "{effect}"
                }
            }
        }
    })
}

fn PetFoodContainer<'a>(cx: Scope<'a, BattleUIState<'a>>, pet_food: Option<&Food>) -> Element<'a> {
    if let Some(food) = pet_food {
        cx.render(rsx! {
            div {
                class: "w3-container",
                h2 {
                    "{food.name}"
                }
                div {
                    class: "w3-panel w3-leftbar",
                    "{food.ability}"
                }
            }
        })
    } else {
        cx.render(rsx! {
            div {
                class: "w3-container",
                "No Food"
            }
        })
    }
}

pub fn PetAttrContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let selected_pet_attr = cx.props.selected_pet_attr.get();
    let Some(selected_pet) = get_selected_pet(cx) else {
        return cx.render(rsx! {
            div {
                class: "w3-container w3-border",
                "No Selected Pet"
            }
        })
    };

    cx.render(rsx! {
        div {
            class: "w3-container w3-border",
            style: "overflow: scroll;",

            if selected_pet_attr == "Stats" {
                PetStatContainer(cx, selected_pet.stats)
            } else if selected_pet_attr == "Effect" {
                PetEffectContainer(cx, &selected_pet.effect)
            } else {
                PetFoodContainer(cx, selected_pet.item.as_ref())
            }
        }
    })
}

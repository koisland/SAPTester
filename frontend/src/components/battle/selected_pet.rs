use dioxus::prelude::*;
use log::info;

use crate::{
    components::battle::{
        ui::BattleUIState,
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
                style: "float: left;",
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
                            attack: if stat_label == "Attack" { *input_stat_value } else { pet_stats.attack },
                            health: if stat_label == "Health" { *input_stat_value } else { pet_stats.health }
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

fn PetStatContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let Some(PetProperty::Stats(pet_stats)) = get_selected_pet_property(cx, "Stats") else {
        return cx.render(rsx! {
            div {
                class: "w3-container",
                h2 { "No Selected Pet" }
            }
        })
    };
    cx.render(rsx! {
        form {
            class: "w3-container",
            h2 { "Stats" }
            LabeledStatInput(cx, "Attack", pet_stats.attack, pet_stats),
            LabeledStatInput(cx, "Health", pet_stats.health, pet_stats)
        }
    })
}

fn EffectPanel<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    effect: &Effect,
    header: Option<String>,
) -> Element<'a> {
    let header = if let Some(header_name) = header {
        cx.render(rsx! { li { h2 { "{header_name}" } } })
    } else {
        None
    };
    cx.render(rsx! {
        div {
            class: "w3-panel w3-leftbar",
            ul {
                class: "w3-ul",
                header
                li {
                    b { "Uses: "}
                    "{effect.uses:?}"
                }
                li {
                    b { "Action: "}
                    "{effect.action} to {effect.target:?} ({effect.position:?})"
                }
                li {
                    b { "Trigger: "}
                    "{effect.trigger.status} ({effect.trigger.position:?})"
                }
            }
        }
    })
}

fn PetEffectContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let (Some(PetProperty::Effect(pet_effects)), Some(PetProperty::Level(pet_lvl))) = (
        get_selected_pet_property(cx, "Effect"),
        get_selected_pet_property(cx, "Level")
    ) else {
        return cx.render(rsx! {
            div {
                class: "w3-container",
                h2 { "No Selected Pet" }
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
                value: "{pet_lvl}",
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
                EffectPanel(cx, effect, None)
            }
        }
    })
}

fn PetFoodContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let Some(PetProperty::Food(pet_food)) = get_selected_pet_property(cx, "Food") else {
        return cx.render(rsx! {
            div {
                class: "w3-container",
                h2 { "No Selected Pet" }
            }
        })
    };
    if let Some(food) = pet_food {
        cx.render(rsx! {
            div {
                class: "w3-container",
                EffectPanel(cx, &food.ability, Some(food.name.to_string()))
            }
        })
    } else {
        cx.render(rsx! {
            div {
                class: "w3-container",
                h2 { "No Selected Food" }
            }
        })
    }
}

pub fn PetAttrContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let selected_pet_attr = cx.props.selected_pet_attr.get();

    cx.render(rsx! {
        div {
            class: "w3-container ",

            if selected_pet_attr == "Stats" {
                PetStatContainer(cx)
            } else if selected_pet_attr == "Effect" {
                PetEffectContainer(cx)
            } else {
                PetFoodContainer(cx)
            }
        }
    })
}

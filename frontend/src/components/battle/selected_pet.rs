use dioxus::prelude::*;
use log::info;

use crate::{
    components::battle::{
        state::{assign_pet_property, get_selected_pet_property},
        ui::BattleUIState,
        utils::{PetProperty, SimpleEffect, SimpleFood},
    },
    utils::get_records::{ATTACK_ICON, HEALTH_ICON},
    RECORDS,
};

fn LabeledStatInput<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    stat_label: &'a str,
    starting_value: usize,
) -> Element<'a> {
    let is_valid_state = use_state(cx, || true);
    let Some(pet_idx) = cx.props.selected_pet_idx.get() else {
        return None;
    };
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
                min: "{1}",
                max: "{50}",
                required: true,
                onchange: move |evt| {
                    if let Ok(input_stat_value) = &evt.data.value.parse::<usize>().map(|value| value.clamp(1, 50)) {
                        is_valid_state.set(true);
                        let stat_value: Option<PetProperty> = match stat_label {
                            "Attack" => Some(PetProperty::Attack(Some(*input_stat_value))),
                            "Health" => Some(PetProperty::Health(Some(*input_stat_value))),
                            _ => None
                        };

                        if let Some(stat_value) = stat_value {
                            if let Err(err) = assign_pet_property(cx, *pet_idx, stat_value) {
                                info!("{err}")
                            }
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
    let (
        Some(PetProperty::Attack(Some(attack))),
        Some(PetProperty::Health(Some(health)))
    ) = (
        get_selected_pet_property(cx, "Attack"),
        get_selected_pet_property(cx, "Health")
    ) else {
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
            LabeledStatInput(cx, "Attack", attack),
            LabeledStatInput(cx, "Health", health)
        }
    })
}

fn EffectPanel<'a>(
    cx: Scope<'a, BattleUIState<'a>>,
    effect: &SimpleEffect,
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
                    "{effect.text}"
                }
                li {
                    b { "Trigger: "}
                    "{effect.trigger}"
                }
            }
        }
    })
}

fn PetEffectContainer<'a>(cx: Scope<'a, BattleUIState<'a>>) -> Element<'a> {
    let (
        Some(PetProperty::Effect(pet_effects)),
        Some(PetProperty::Level(pet_lvl)),
        Some(pet_idx)
    ) = (
        get_selected_pet_property(cx, "Effect"),
        get_selected_pet_property(cx, "Level"),
        cx.props.selected_pet_idx.get()
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
                (1..=3).map(|lvl| {
                    rsx! {
                        option {
                            value: "{lvl}",
                            onclick: move |_| {
                                if let Err(err) = assign_pet_property(cx, *pet_idx, PetProperty::Level(lvl)) {
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
    let Some(PetProperty::Food(Some(pet_food))) = get_selected_pet_property(cx, "Food") else {
        return cx.render(rsx! {
            div {
                class: "w3-container",
                h2 { "No Food for Selected Pet" }
            }
        })
    };
    if let Some(food) = RECORDS
        .get()
        .and_then(|records| records.get("Foods").and_then(|foods| foods.get(&pet_food)))
        .and_then(|food_val| TryInto::<SimpleFood>::try_into(food_val).ok())
    {
        cx.render(rsx! {
            div {
                class: "w3-container",
                EffectPanel(cx, &food.effect, Some(food.name.to_string()))
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

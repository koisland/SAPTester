use dioxus::prelude::*;
use dioxus_router::{Route, Router};

use crate::components::{battle::Battle, footer::Footer, home::Home, nav::Nav};

pub fn AppRoutes(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Nav {},
            for _ in 0..3 {
                br {}
            }
            Route { to: "/home" , Home {} },
            Route { to: "/battle" , Battle {} },
            Route { to: "/about" }
            Footer {}
        }
    })
}

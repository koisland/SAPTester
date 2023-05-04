use dioxus::prelude::*;
use dioxus_router::{Route, Router};

use crate::components::{footer::Footer, home::Home, nav::Nav};

pub fn AppRoutes(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Nav {},
            br {}
            br {}
            br {}

            Route { to: "/home" , Home {} },
            Route { to: "/battle" },
            Route { to: "/about" }
            Footer {}
        }
    })
}

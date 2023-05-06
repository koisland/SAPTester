// use dioxus::{
//     router::{Route, Router},
//     prelude::*
// };

// use crate::components::{battle::ui::Battle, footer::Footer, home::Home, nav::Nav};

// pub fn AppRoutes(cx: Scope) -> Element {
//     cx.render(rsx! {
//         Router {
//             Nav {},
//             br {}
//             br {}
//             br {}

//             Route { to: "/home" , Home {} },
//             Route { to: "/battle", Battle {}  },
//             Route { to: "/about" }
//             Footer {}
//         }
//     })
// }

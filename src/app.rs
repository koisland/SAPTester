use dioxus::prelude::*;
use sir::{global_css, AppStyle};

use crate::components::routes::AppRoutes;

pub fn app(cx: Scope) -> Element {
    // https://www.w3schools.com/w3css/tryit.asp?filename=tryw3css_templates_analytics&stacked=h
    cx.render(rsx!{
        global_css!(r#"
            html,body,h1,h2,h3,h4,h5 {font-family: "Raleway", sans-serif;}
        "#)
        link {
            rel: "stylesheet",
            href: "https://www.w3schools.com/w3css/4/w3.css"
        }
        link {
            rel:"stylesheet",
            href:"https://fonts.googleapis.com/css?family=Raleway"
        }
        link {
            rel:"stylesheet",
            href:"https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
        }
        AppStyle {},
        body {
            class: "w3-light-grey",
            AppRoutes {}
        }

    })
}

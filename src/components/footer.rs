// src/components/footer.rs
use dioxus::prelude::*;

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            div { class: "container",
                a { class: "logo-font", href: "/", "conduit" }
                span { class: "attribution",
                    "\n        An interactive learning project from "
                    a { href: "https://thinkster.io", "Thinkster" }
                    ". Code &\n        design licensed under MIT.\n      "
                }
            }
        }

    }
}
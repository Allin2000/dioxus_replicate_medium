use dioxus::prelude::*;

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Header() -> Element {
    rsx! {
        nav { class: "navbar navbar-light",
                div { class: "container",
                    a { class: "navbar-brand", href: "/", "conduit" }
                    ul { class: "nav navbar-nav pull-xs-right",
                        li { class: "nav-item",
                            a { class: "nav-link active", href: "/", "Home" }
                        }
                        li { class: "nav-item",
                            a { class: "nav-link", href: "/login", "Sign in" }
                        }
                        li { class: "nav-item",
                            a { class: "nav-link", href: "/register", "Sign up" }
                        }
                    }
                }
        }

    }
}
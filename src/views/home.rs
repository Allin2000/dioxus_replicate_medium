// use crate::components::Hero;
// use dioxus::prelude::*;

// #[component]
// pub fn Home() -> Element {
//     rsx! {
//         Hero {}

//     }
// }

// use crate::components::Hero;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "home-page",
        div { class: "banner",
            div { class: "container",
                h1 { class: "logo-font", "conduit" }
                p { "A place to share your knowledge." }
            }
        }
        div { class: "container page",
            div { class: "row",
                div { class: "col-md-9",
                    div { class: "feed-toggle",
                        ul { class: "nav nav-pills outline-active",
                            li { class: "nav-item",
                                a { class: "nav-link", href: "", "Your Feed" }
                            }
                            li { class: "nav-item",
                                a {
                                    class: "nav-link active",
                                    href: "",
                                    "Global Feed"
                                }
                            }
                        }
                    }
                    div { class: "article-preview",
                        div { class: "article-meta",
                            a { href: "/profile/eric-simons",
                                img { src: "http://i.imgur.com/Qr71crq.jpg" }
                            }
                            div { class: "info",
                                a {
                                    class: "author",
                                    href: "/profile/eric-simons",
                                    "Eric Simons"
                                }
                                span { class: "date", "January 20th" }
                            }
                            button { class: "btn btn-outline-primary btn-sm pull-xs-right",
                                i { class: "ion-heart" }
                                " 29\n              "
                            }
                        }
                        a {
                            class: "preview-link",
                            href: "/article/how-to-build-webapps-that-scale",
                            h1 { "How to build webapps that scale" }
                            p { "This is the description for the post." }
                            span { "Read more..." }
                            ul { class: "tag-list",
                                li { class: "tag-default tag-pill tag-outline",
                                    "realworld"
                                }
                                li { class: "tag-default tag-pill tag-outline",
                                    "implementations"
                                }
                            }
                        }
                    }
                    div { class: "article-preview",
                        div { class: "article-meta",
                            a { href: "/profile/albert-pai",
                                img { src: "http://i.imgur.com/N4VcUeJ.jpg" }
                            }
                            div { class: "info",
                                a {
                                    class: "author",
                                    href: "/profile/albert-pai",
                                    "Albert Pai"
                                }
                                span { class: "date", "January 20th" }
                            }
                            button { class: "btn btn-outline-primary btn-sm pull-xs-right",
                                i { class: "ion-heart" }
                                " 32\n              "
                            }
                        }
                        a {
                            class: "preview-link",
                            href: "/article/the-song-you",
                            h1 {
                                "The song you won't ever stop singing. No matter how hard you try."
                            }
                            p { "This is the description for the post." }
                            span { "Read more..." }
                            ul { class: "tag-list",
                                li { class: "tag-default tag-pill tag-outline",
                                    "realworld"
                                }
                                li { class: "tag-default tag-pill tag-outline",
                                    "implementations"
                                }
                            }
                        }
                    }
                    ul { class: "pagination",
                        li { class: "page-item active",
                            a { class: "page-link", href: "", "1" }
                        }
                        li { class: "page-item",
                            a { class: "page-link", href: "", "2" }
                        }
                    }
                }
                div { class: "col-md-3",
                    div { class: "sidebar",
                        p { "Popular Tags" }
                        div { class: "tag-list",
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "programming"
                            }
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "javascript"
                            }
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "emberjs"
                            }
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "angularjs"
                            }
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "react"
                            }
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "mean"
                            }
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "node"
                            }
                            a {
                                class: "tag-pill tag-default",
                                href: "",
                                "rails"
                            }
                        }
                    }
                }
            }
        }
    }
    }
}



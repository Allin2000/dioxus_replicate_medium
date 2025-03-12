use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "profile-page",
                div { class: "user-info",
                    div { class: "container",
                        div { class: "row",
                            div { class: "col-xs-12 col-md-10 offset-md-1",
                                img {
                                    class: "user-img",
                                    src: "http://i.imgur.com/Qr71crq.jpg",
                                }
                                h4 { "Eric Simons" }
                                p {
                                    "\n              Cofounder @GoThinkster, lived in Aol's HQ for a few months, kinda looks like Peeta from\n              the Hunger Games\n            "
                                }
                                button { class: "btn btn-sm btn-outline-secondary action-btn",
                                    i { class: "ion-plus-round" }
                                    "\n              \u{a0} Follow Eric Simons\n            "
                                }
                                button { class: "btn btn-sm btn-outline-secondary action-btn",
                                    i { class: "ion-gear-a" }
                                    "\n              \u{a0} Edit Profile Settings\n            "
                                }
                            }
                        }
                    }
                }
                div { class: "container",
                    div { class: "row",
                        div { class: "col-xs-12 col-md-10 offset-md-1",
                            div { class: "articles-toggle",
                                ul { class: "nav nav-pills outline-active",
                                    li { class: "nav-item",
                                        a {
                                            class: "nav-link active",
                                            href: "",
                                            "My Articles"
                                        }
                                    }
                                    li { class: "nav-item",
                                        a { class: "nav-link", href: "", "Favorited Articles" }
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
                                    href: "/article/how-to-buil-webapps-that-scale",
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
                                            "Music"
                                        }
                                        li { class: "tag-default tag-pill tag-outline",
                                            "Song"
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
                    }
                }
        }
    }
}




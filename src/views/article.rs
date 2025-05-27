// src/views/article.rs
use dioxus::prelude::*;

#[component]
pub fn Article() -> Element {
    rsx! {
        div { class: "article-page",
        div { class: "banner",
            div { class: "container",
                h1 { "How to build webapps that scale" }
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
                    button { class: "btn btn-sm btn-outline-secondary",
                        i { class: "ion-plus-round" }
                        "\n            \u{a0} Follow Eric Simons "
                        span { class: "counter", "(10)" }
                    }
                    "\n          \u{a0}\u{a0}\n          "
                    button { class: "btn btn-sm btn-outline-primary",
                        i { class: "ion-heart" }
                        "\n            \u{a0} Favorite Post "
                        span { class: "counter", "(29)" }
                    }
                    button { class: "btn btn-sm btn-outline-secondary",
                        i { class: "ion-edit" }
                        " Edit Article\n          "
                    }
                    button { class: "btn btn-sm btn-outline-danger",
                        i { class: "ion-trash-a" }
                        " Delete Article\n          "
                    }
                }
            }
        }
        div { class: "container page",
            div { class: "row article-content",
                div { class: "col-md-12",
                    p {
                        "\n            Web development technologies have evolved at an incredible clip over the past few years.\n          "
                    }
                    h2 { id: "introducing-ionic", "Introducing RealWorld." }
                    p { "It's a great solution for learning how other frameworks work." }
                    ul { class: "tag-list",
                        li { class: "tag-default tag-pill tag-outline", "realworld" }
                        li { class: "tag-default tag-pill tag-outline", "implementations" }
                    }
                }
            }
            hr {}
            div { class: "article-actions",
                div { class: "article-meta",
                    a { href: "profile.html",
                        img { src: "http://i.imgur.com/Qr71crq.jpg" }
                    }
                    div { class: "info",
                        a { class: "author", href: "", "Eric Simons" }
                        span { class: "date", "January 20th" }
                    }
                    button { class: "btn btn-sm btn-outline-secondary",
                        i { class: "ion-plus-round" }
                        "\n            \u{a0} Follow Eric Simons\n          "
                    }
                    "\n          \u{a0}\n          "
                    button { class: "btn btn-sm btn-outline-primary",
                        i { class: "ion-heart" }
                        "\n            \u{a0} Favorite Article "
                        span { class: "counter", "(29)" }
                    }
                    button { class: "btn btn-sm btn-outline-secondary",
                        i { class: "ion-edit" }
                        " Edit Article\n          "
                    }
                    button { class: "btn btn-sm btn-outline-danger",
                        i { class: "ion-trash-a" }
                        " Delete Article\n          "
                    }
                }
            }
            div { class: "row",
                div { class: "col-xs-12 col-md-8 offset-md-2",
                    form { class: "card comment-form",
                        div { class: "card-block",
                            textarea {
                                class: "form-control",
                                placeholder: "Write a comment...",
                                rows: "3",
                            }
                        }
                        div { class: "card-footer",
                            img {
                                class: "comment-author-img",
                                src: "http://i.imgur.com/Qr71crq.jpg",
                            }
                            button { class: "btn btn-sm btn-primary", "Post Comment" }
                        }
                    }
                    div { class: "card",
                        div { class: "card-block",
                            p { class: "card-text",
                                "\n                With supporting text below as a natural lead-in to additional content.\n              "
                            }
                        }
                        div { class: "card-footer",
                            a {
                                class: "comment-author",
                                href: "/profile/author",
                                img {
                                    class: "comment-author-img",
                                    src: "http://i.imgur.com/Qr71crq.jpg",
                                }
                            }
                            "\n              \u{a0}\n              "
                            a {
                                class: "comment-author",
                                href: "/profile/jacob-schmidt",
                                "Jacob Schmidt"
                            }
                            span { class: "date-posted", "Dec 29th" }
                        }
                    }
                    div { class: "card",
                        div { class: "card-block",
                            p { class: "card-text",
                                "\n                With supporting text below as a natural lead-in to additional content.\n              "
                            }
                        }
                        div { class: "card-footer",
                            a {
                                class: "comment-author",
                                href: "/profile/author",
                                img {
                                    class: "comment-author-img",
                                    src: "http://i.imgur.com/Qr71crq.jpg",
                                }
                            }
                            "\n              \u{a0}\n              "
                            a {
                                class: "comment-author",
                                href: "/profile/jacob-schmidt",
                                "Jacob Schmidt"
                            }
                            span { class: "date-posted", "Dec 29th" }
                            span { class: "mod-options",
                                i { class: "ion-trash-a" }
                            }
                        }
                    }
                }
            }
        }
    }
}
}




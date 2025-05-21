use dioxus::prelude::*;
use crate::components::TagList;
use crate::components::ArticleList;

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
                    
                    ArticleList {}

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
                                TagList {  }
                            }
                        }
                }
            }
        }
    }
    }
}



// use crate::components::Hero;
// use dioxus::prelude::*;

// #[component]
// pub fn Home() -> Element {
//     rsx! {
//         Hero {}

//     }
// }

// use crate::components::Hero;



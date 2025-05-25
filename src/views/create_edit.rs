// src/api/create_edit.rs
use dioxus::prelude::*;

#[component]
pub fn Create_edit() -> Element {
    rsx! {
        div { class: "editor-page",
        div { class: "container page",
            div { class: "row",
                div { class: "col-md-10 offset-md-1 col-xs-12",
                    ul { class: "error-messages",
                        li { "That title is required" }
                    }
                    form {
                        fieldset {
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Article Title",
                                    r#type: "text",
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control",
                                    placeholder: "What's this article about?",
                                    r#type: "text",
                                }
                            }
                            fieldset { class: "form-group",
                                textarea {
                                    class: "form-control",
                                    placeholder: "Write your article (in markdown)",
                                    rows: "8",
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control",
                                    placeholder: "Enter tags",
                                    r#type: "text",
                                }
                                div { class: "tag-list",
                                    span { class: "tag-default tag-pill",
                                        i { class: "ion-close-round" }
                                        " tag "
                                    }
                                }
                            }
                            button {
                                class: "btn btn-lg pull-xs-right btn-primary",
                                r#type: "button",
                                "\n                Publish Article\n              "
                            }
                        }
                    }
                }
            }
        }
    }
}
}



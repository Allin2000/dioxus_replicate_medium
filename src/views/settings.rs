use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "settings-page",
        div { class: "container page",
            div { class: "row",
                div { class: "col-md-6 offset-md-3 col-xs-12",
                    h1 { class: "text-xs-center", "Your Settings" }
                    ul { class: "error-messages",
                        li { "That name is required" }
                    }
                    form {
                        fieldset {
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control",
                                    placeholder: "URL of profile picture",
                                    r#type: "text",
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Your Name",
                                    r#type: "text",
                                }
                            }
                            fieldset { class: "form-group",
                                textarea {
                                    class: "form-control form-control-lg",
                                    placeholder: "Short bio about you",
                                    rows: "8",
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "Email",
                                    r#type: "text",
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    placeholder: "New Password",
                                    r#type: "password",
                                }
                            }
                            button { class: "btn btn-lg btn-primary pull-xs-right",
                                "Update Settings"
                            }
                        }
                    }
                    hr {}
                    button { class: "btn btn-outline-danger", "Or click here to logout." }
                }
            }
        }
        }
    }
}




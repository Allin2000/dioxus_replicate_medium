use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    rsx! {
        div { class: "auth-page",
        div { class: "container page",
            div { class: "row",
                div { class: "col-md-6 offset-md-3 col-xs-12",
                    h1 { class: "text-xs-center", "Sign up" }
                    p { class: "text-xs-center",
                        a { href: "/login", "Have an account?" }
                    }
                    ul { class: "error-messages",
                        li { "That email is already taken" }
                    }
                    form {
                        fieldset { class: "form-group",
                            input {
                                class: "form-control form-control-lg",
                                placeholder: "Username",
                                r#type: "text",
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
                                placeholder: "Password",
                                r#type: "password",
                            }
                        }
                        button { class: "btn btn-lg btn-primary pull-xs-right",
                            "Sign up"
                        }
                    }
                }
            }
        }
    }
}
}



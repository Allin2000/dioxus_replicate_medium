use dioxus::prelude::*;

// use components::Navbar;
use components::Header;
use components::Footer;

use stores::app_state::{AppState, AuthStatus};

// 导入 gloo-timers 和 web_sys
use gloo_timers::future::TimeoutFuture; // <-- 新增
use web_sys::console; // <-- 新增

// use views::{Blog, Home};
use views::{Home,Login,Register,Profile,Settings,Create_edit,Article};

mod components;
mod views;
mod api;
mod stores;




#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[layout(Wrapper)]
    // #[layout(Footer)]
        #[route("/")]
        Home {},
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
        #[route("/profile")]
        Profile {},
        #[route("/settings")]
        Settings {},
        #[route("/create_edit")]
        Create_edit {},
        #[route("/article")]
        Article {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}


#[component]
fn Wrapper() -> Element{
    rsx! {
        Header {}
        Outlet::<Route> {}
        Footer {}
    }
}


// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}



#[component]
fn App() -> Element {
    // Build cool things ✌️

    let app_state = AppState::new();

        // **Simulate a login after 2 seconds for demonstration purposes.**
    // In a real application, this would be updated after a successful API login call.
    // use_effect(move || {
    //     let auth_status_signal = app_state.auth_status; // Get a clone of the signal for the async block
    //     spawn(async move {
    //         dioxus::tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    //         auth_status_signal.set(AuthStatus::LoggedIn); // Set the global state to LoggedIn
    //         web_sys::console::log_1(&"Simulated login state set to LoggedIn!".into());
    //     });
    // });

        // 2. 模拟登录状态改变 (2 秒后模拟登录)
    use_effect(move || {
        let mut auth_status_signal = app_state.auth_status; // 克隆信号，以便在异步块中使用
        spawn(async move {
            TimeoutFuture::new(2000).await; // 使用 gloo-timers 实现 2 秒延迟
            auth_status_signal.set(AuthStatus::LoggedIn); // 设置全局状态为已登录
            console::log_1(&"Simulated login state set to LoggedIn!".into()); // 使用 web_sys 打印到控制台
        });
    });



    use_context_provider(|| app_state.clone()); // `use_context_provider` 接收一个返回上下文值的闭包


    rsx! {
        // Global app resources
        head {
            document::Meta { charset: "utf-8" }
            document::Title { "Conduit" }
            // document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: "//code.ionicframework.com/ionicons/2.0.1/css/ionicons.min.css" }
            document::Link { rel: "stylesheet", href: "//fonts.googleapis.com/css?family=Titillium+Web:700|Source+Serif+Pro:400,700|Merriweather+Sans:400,700|Source+Sans+Pro:400,300,600,700,300italic,400italic,600italic,700italic" }
            document::Link { rel: "stylesheet", href: "//demo.productionready.io/main.css" }
            // document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        }


        Router::<Route> {}
    }
}


// #[component]
// fn App() -> Element {
//     // Build cool things ✌️

//     rsx! {
//         // Global app resources
//         document::Link { rel: "icon", href: FAVICON }
//         document::Link { rel: "stylesheet", href: MAIN_CSS }

//         Router::<Route> {}
//     }
// }
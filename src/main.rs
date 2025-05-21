use dioxus::prelude::*;

use components::Header;
use components::Footer;

use stores::app_state::{AppState, AuthStatus};

// 导入 gloo-timers 和 web_sys
use gloo_timers::future::TimeoutFuture; // <-- 新增
use web_sys::console; // <-- 新增

use views::{Home,Login,Register,Profile,Settings,Create_edit,Article};

mod components;
mod views;
mod api;
mod stores;




#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[layout(Wrapper)]
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
}


#[component]
fn Wrapper() -> Element{

    // FIX 1: 创建一个 Signal<AppState>，而不是直接 AppState::new()
    let app_state_signal = use_signal(|| AppState::new());

    // FIX 2: use_effect 中捕获 app_state_signal，并操作其内部的 auth_status 信号
    use_effect(move || {
        // 使用 .read() 获取 Signal<AppState> 内部的 AppState 引用，然后访问 auth_status
        // auth_status 本身就是 Signal，所以可以直接 .clone() 给异步块
        let mut auth_status_inner_signal = app_state_signal.read().auth_status; // auth_status 自身就是 Signal

        spawn(async move {
            TimeoutFuture::new(200).await;
            auth_status_inner_signal.set(AuthStatus::LoggedIn); // 直接设置 auth_status
            console::log_1(&"Simulated login state set to LoggedIn!".into());
        });
    });

    // FIX 3: use_context_provider 提供 Signal<AppState>
    use_context_provider(|| app_state_signal); // 直接提供 app_state_signal

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


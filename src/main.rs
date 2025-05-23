use dioxus::prelude::*;

use components::Header;
use components::Footer;

use stores::app_state::{AppState, AuthStatus};

use crate::views::GlobalFeed;
use crate::views::YourFeed;


// // 导入 gloo-timers 和 web_sys
// use gloo_timers::future::TimeoutFuture; // <-- 新增
// use web_sys::console; // <-- 新增

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

        #[route("/global")]
        GlobalFeed {},

        #[route("/your")]
        YourFeed {},

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
        // #[route("/article/:slug")] // Added :slug for individual article view
        // Article { slug: String },
}




#[component]
fn Wrapper() -> Element{

    //  let app_state_signal = use_context::<Signal<AppState>>();
 

    // // 2. 将这些 Signal 重新提供给 Wrapper 的子组件（包括 Outlet 渲染的路由组件）
    // use_context_provider(|| app_state_signal);

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


    let app_state_signal = use_signal(|| AppState::new());
    use_context_provider(|| app_state_signal); // 直接提供 app_state_signal

    

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
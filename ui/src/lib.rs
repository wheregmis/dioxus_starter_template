#[cfg(feature = "server")]
mod backend_ext;

pub mod server_fns;

use crate::server_fns::counter::get_counter;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("./assets/favicon.ico");
const MAIN_CSS: Asset = asset!("./assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let current_counter = use_resource(move || async move { get_counter().await });

    rsx! {
        match &*current_counter.read_unchecked() {
            Some(Ok(counter)) => rsx! { div { "Counter yoho: {counter}" } },
            Some(Err(e)) => rsx! { p { "Loading counter failed, {e}" } },
            None =>  rsx! { p { "Loading..." } }
        }
    }
}

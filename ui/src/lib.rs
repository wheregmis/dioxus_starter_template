#[cfg(feature = "server")]
mod backend_ext;

pub mod features;

use crate::features::counter::CounterDisplay;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("./assets/favicon.ico");

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "container mx-auto p-4",
            h1 {
                class: "text-2xl font-bold mb-4",
                "Counter App"
            }
            CounterDisplay {}
        }
    }
}

pub mod features;

use crate::features::auth::AuthForm;
use crate::features::counter::CounterDisplay;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/home")]
    Home {},
}

const FAVICON: Asset = asset!("./assets/favicon.ico");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Script { src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" }
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
                "Dioxus Starter Template"
            }

            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                div {
                    class: "bg-gray-50 p-6 rounded-lg",
                    h2 { class: "text-xl font-semibold mb-4", "Counter Feature" }
                    CounterDisplay {}
                }

                div {
                    class: "bg-gray-50 p-6 rounded-lg",
                    h2 { class: "text-xl font-semibold mb-4", "Auth Feature" }
                    AuthForm {}
                }
            }
        }
    }
}

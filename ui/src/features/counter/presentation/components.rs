use crate::features::counter::domain::GetCounterUseCase;
use dioxus::prelude::*;

#[component]
pub fn CounterDisplay() -> Element {
    let current_counter = use_resource(move || async move { GetCounterUseCase::execute().await });

    rsx! {
        match &*current_counter.read_unchecked() {
            Some(Ok(counter)) => rsx! {
                div {
                    class: "p-4",
                    "Counter value: {counter.value}"
                }
            },
            Some(Err(e)) => rsx! {
                p {
                    class: "text-red-500",
                    "Loading counter failed: {e}"
                }
            },
            None => rsx! {
                p {
                    class: "text-gray-500",
                    "Loading..."
                }
            }
        }
    }
}

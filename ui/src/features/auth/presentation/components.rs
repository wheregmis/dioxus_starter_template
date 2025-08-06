use dioxus::prelude::*;

use crate::features::GetCurrentUserUseCase;

#[component]
pub fn AuthForm() -> Element {
    let user = use_resource(move || async move { GetCurrentUserUseCase::execute().await });

    rsx! {
        div {
            "AuthForm"
            match &*user.read_unchecked() {
                Some(Ok(user)) => rsx! {
                    div {
                        "User: {user.email}"
                    }
                },
                Some(Err(e)) => rsx! {
                    div {
                        "Error: {e}"
                    }
                },
                None => rsx! {
                    div {
                        "Loading..."
                    }
                }
            }
        }
    }
}

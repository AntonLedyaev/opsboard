use leptos::{component, create_resource, spawn_local, view, IntoView, Suspense};
use leptos::prelude::*;
use crate::api::{fetch_health, fetch_jobs};

#[component]
pub fn HomePage() -> impl IntoView {
    let data = create_resource(
        || (),
        |_| async { fetch_health().await.ok() }
    );

    let result = create_signal(None::<String>);

    let on_click = move |_| {
        let set_result = result.1;

        spawn_local(async move {
            match fetch_jobs().await {
                Ok(data) => set_result.set(Some(data)),
                Err(_) => set_result.set(Some("Ошибка".into())),
            }
        });
    };

    view! {
        <h1>"Leptos SPA"</h1>
         <button on:click=on_click>
            "Get Jobs"
        </button>

        <Suspense fallback=|| view! { <p>"Загрузка..."</p> }>
            {move || data.get().map(|res| match res {
                Some(text) => view! { <p>{text}</p> }.into_view(),
                None => view! { <p>"Ошибка загрузки"</p> }.into_view(),
            })}
            {move || result.0.get().map(|text| view! {
                <p>{text}</p>
            })}
        </Suspense>
    }
}
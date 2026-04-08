use std::time::Duration;
use leptos::{component, event_target_value, spawn_local, view, CollectView, Effect, IntoView, Suspense};
use leptos::prelude::*;
use gloo_timers::future::sleep;

use crate::api::{fetch_jobs, fetch_jobs_post, run_jobs, CreateJobRequestPayload, JobView};

#[component]
pub fn HomePage() -> impl IntoView {
    let (job_name_input, set_job_name_input) = create_signal(String::new());
    let (job_created, set_job_created) = create_signal(None::<String>);
    let (run_result, set_run_result) = create_signal(None::<String>);

    let (jobs, set_jobs) = create_signal(Vec::<JobView>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    Effect::new(move |_| {
        spawn_local({
            let set_jobs = set_jobs;
            let set_loading = set_loading;
            let set_error = set_error;

            async move {
                loop {
                    set_loading.set(true);

                    match fetch_jobs().await {
                        Ok(data) => {
                            set_jobs.set(data);
                            set_error.set(None);
                        }
                        Err(e) => {
                            set_error.set(Some(format!("Ошибка запроса: {e}")));
                        }
                    }

                    set_loading.set(false);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        });
    });


    let on_submit = move |_| {
        let payload = CreateJobRequestPayload {
            name: job_name_input.get(),
        };
        let set_result = set_job_created.clone();

        spawn_local(async move {
            match fetch_jobs_post(payload).await {
                Ok(data) => {
                    set_job_created.set(Some(data));
                }
                Err(_) => set_result.set(Some("Ошибка запроса".into())),
            }
        });
    };


    let on_run_jobs_click = move |_| {
        spawn_local(async move {
            match run_jobs().await {
                Ok(data) => {
                    set_run_result.set(Some(data));
                }
                Err(_) => set_run_result.set(Some("Ошибка запроса".into())),
            }
        })
    };

    view! {
        <h1>"OpsBoard"</h1>

        <input
            type="text"
            placeholder="Print Job Name"
            on:input=move |ev| {
                set_job_name_input.set(event_target_value(&ev));
            }
        />

        <button on:click=on_submit>
            "Create Job"
        </button>

        <button on:click=on_run_jobs_click>
            "Run Jobs"
        </button>


        <Suspense fallback=|| view! { <p>"Загрузка..."</p> }>
                {move || {
                    jobs.get()
                        .into_iter()
                        .map(|job| {
                            view! {
                                <li>
                                    <b>{job.name}</b>
                                    " | id: " {job.id}
                                    " | status: " {job.status}
                                    " | retry_count: " {job.retry_count}
                                </li>
                            }
                        })
                        .collect_view()
                }}

            {move || job_created.get().map(|r| view! {
                <p>{r}</p>
            })}
            {move || run_result.get().map(|text| view! {
                <p>{text}</p>
            })}
        </Suspense>
    }
}
use std::time::Duration;

use gloo_timers::future::sleep;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::{component, spawn_local, view, Effect, IntoView};

use crate::api::{
    delete_job, fetch_jobs, fetch_jobs_post, run_jobs, CreateJobRequestPayload,
    DeleteJobRequestPayload, JobView,
};
use crate::components::header_section::HeaderSection;
use crate::components::jobs_table::JobsTable;
use crate::components::quick_actions::QuickActions;
use crate::components::summary_cards::{JobSummary, SummaryCards};

#[component]
pub fn HomePage() -> impl IntoView {
    let (job_name_input, set_job_name_input) = create_signal(String::new());
    let (job_created, set_job_created) = create_signal(None::<String>);
    let (run_result, set_run_result) = create_signal(None::<String>);

    let (jobs, set_jobs) = create_signal(Vec::<JobView>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    let refresh_jobs = move || {
        let set_jobs = set_jobs;
        let set_loading = set_loading;
        let set_error = set_error;

        spawn_local(async move {
            set_loading.set(true);

            match fetch_jobs().await {
                Ok(data) => {
                    set_jobs.set(data);
                    set_error.set(None);
                }
                Err(err) => {
                    set_error.set(Some(format!("Request error: {err}")));
                }
            }

            set_loading.set(false);
        });
    };

    Effect::new(move |_| {
        refresh_jobs();

        spawn_local({
            let refresh_jobs = refresh_jobs;

            async move {
                loop {
                    sleep(Duration::from_secs(3)).await;
                    refresh_jobs();
                }
            }
        });
    });

    let summary = Memo::new(move |_| JobSummary::from_jobs(&jobs.get()));

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let name = job_name_input.get().trim().to_string();
        if name.is_empty() {
            set_job_created.set(Some("Job name is required".into()));
            return;
        }

        let payload = CreateJobRequestPayload { name };
        let set_job_created = set_job_created;
        let set_job_name_input = set_job_name_input;
        let refresh_jobs = refresh_jobs;

        spawn_local(async move {
            match fetch_jobs_post(payload).await {
                Ok(data) => {
                    set_job_created.set(Some(data));
                    set_job_name_input.set(String::new());
                    refresh_jobs();
                }
                Err(err) => set_job_created.set(Some(format!("Request error: {err}"))),
            }
        });
    };

    let on_run_jobs_click = move |_| {
        let refresh_jobs = refresh_jobs;

        spawn_local(async move {
            match run_jobs().await {
                Ok(data) => {
                    set_run_result.set(Some(data));
                    refresh_jobs();
                }
                Err(err) => set_run_result.set(Some(format!("Request error: {err}"))),
            }
        })
    };

    let on_refresh_click = move |_| {
        refresh_jobs();
    };

    let on_delete = move |id: u32| {
        let refresh_jobs = refresh_jobs;
        let set_run_result = set_run_result;

        spawn_local(async move {
            let payload = DeleteJobRequestPayload { id };

            match delete_job(payload).await {
                Ok(message) => {
                    set_run_result.set(Some(message));
                    refresh_jobs();
                }
                Err(err) => set_run_result.set(Some(format!("Request error: {err}"))),
            }
        });
    };

    view! {
        <div class="min-h-screen bg-[radial-gradient(circle_at_top_left,rgba(14,165,233,0.16),transparent_30%),radial-gradient(circle_at_top_right,rgba(34,197,94,0.12),transparent_25%),linear-gradient(180deg,#f8fbff_0%,#eef4f8_100%)] px-4 py-8 md:px-5 md:py-10">
            <div class="mx-auto flex max-w-[1180px] flex-col gap-6">
                <HeaderSection loading=loading error=error />

                <QuickActions
                    job_name=job_name_input
                    set_job_name=set_job_name_input
                    on_submit=on_submit
                    on_run_queue=on_run_jobs_click
                    on_refresh=on_refresh_click
                    job_created=job_created
                    run_result=run_result
                />

                <SummaryCards summary=summary />

                <JobsTable jobs=jobs loading=loading on_delete=on_delete />
            </div>
        </div>
    }
}

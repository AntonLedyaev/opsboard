use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::{component, view, For, IntoView, Suspense};

use crate::api::JobView;

fn status_badge_class(status: &str) -> &'static str {
    match status {
        "Queued" => "inline-flex min-w-[88px] items-center justify-center rounded-full bg-amber-100 px-3 py-2 text-xs font-extrabold tracking-[0.04em] text-amber-800",
        "Running" => "inline-flex min-w-[88px] items-center justify-center rounded-full bg-blue-100 px-3 py-2 text-xs font-extrabold tracking-[0.04em] text-blue-700",
        "Done" => "inline-flex min-w-[88px] items-center justify-center rounded-full bg-emerald-100 px-3 py-2 text-xs font-extrabold tracking-[0.04em] text-emerald-700",
        "Failed" => "inline-flex min-w-[88px] items-center justify-center rounded-full bg-rose-100 px-3 py-2 text-xs font-extrabold tracking-[0.04em] text-rose-700",
        _ => "inline-flex min-w-[88px] items-center justify-center rounded-full bg-slate-200 px-3 py-2 text-xs font-extrabold tracking-[0.04em] text-slate-600",
    }
}

#[component]
pub fn JobsTable<OnDelete>(
    jobs: ReadSignal<Vec<JobView>>,
    loading: ReadSignal<bool>,
    on_delete: OnDelete,
) -> impl IntoView
where
    OnDelete: Fn(u32) + Copy + 'static,
{
    view! {
        <section class="rounded-[22px] border border-slate-200/70 bg-white/90 p-6 shadow-[0_24px_60px_rgba(15,23,42,0.08)] backdrop-blur-sm md:p-7">
            <div class="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
                <div>
                    <h2 class="m-0 text-2xl font-bold text-slate-900">"Jobs"</h2>
                    <p class="mt-1 text-sm text-slate-500">"Airflow-like overview of queued and completed work"</p>
                </div>
            </div>

            <Suspense fallback=|| view! { <div class="px-5 py-8 text-center text-slate-500">"Loading jobs..."</div> }>
                <div class="mt-5 overflow-x-auto rounded-[18px] border border-slate-200 bg-white">
                    <table class="w-full">
                        <thead class="bg-slate-50">
                            <tr>
                                <th class="px-4 py-4 text-left text-xs font-bold uppercase tracking-[0.08em] text-slate-500">"ID"</th>
                                <th class="px-4 py-4 text-left text-xs font-bold uppercase tracking-[0.08em] text-slate-500">"Name"</th>
                                <th class="px-4 py-4 text-left text-xs font-bold uppercase tracking-[0.08em] text-slate-500">"Status"</th>
                                <th class="px-4 py-4 text-left text-xs font-bold uppercase tracking-[0.08em] text-slate-500">"Retry Count"</th>
                                <th class="px-4 py-4 text-left text-xs font-bold uppercase tracking-[0.08em] text-slate-500">"Actions"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                let current_jobs = jobs.get();

                                if current_jobs.is_empty() && !loading.get() {
                                    view! {
                                        <tr>
                                            <td class="px-4 py-8" colspan="5">
                                                <div class="text-center text-slate-500">
                                                    "No jobs yet. Create a job from the quick actions panel."
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                        .into_view()
                                } else {
                                    view! {
                                        <For
                                            each=move || jobs.get()
                                            key=|job| job.id
                                            children=move |job| {
                                                let job_id = job.id;
                                                let delete_click = move |_ev: MouseEvent| on_delete(job_id);

                                                view! {
                                                    <tr class="border-b border-slate-200 last:border-b-0 hover:bg-sky-50/40">
                                                        <td class="px-4 py-4 text-sm text-slate-700">{job.id}</td>
                                                        <td class="px-4 py-4 text-sm font-bold text-slate-900">{job.name}</td>
                                                        <td class="px-4 py-4 text-sm text-slate-700">
                                                            <span class=status_badge_class(&job.status)>
                                                                {job.status}
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-4 text-sm text-slate-700">{job.retry_count}</td>
                                                        <td class="px-4 py-4">
                                                            <button
                                                                class="rounded-2xl bg-rose-100 px-4 py-2.5 text-sm font-bold text-rose-700 transition hover:-translate-y-0.5 hover:bg-rose-200"
                                                                type="button"
                                                                on:click=delete_click
                                                            >
                                                                "Delete Job"
                                                            </button>
                                                        </td>
                                                    </tr>
                                                }
                                            }
                                        />
                                    }
                                        .into_view()
                                }
                            }}
                        </tbody>
                    </table>
                </div>
            </Suspense>
        </section>
    }
}

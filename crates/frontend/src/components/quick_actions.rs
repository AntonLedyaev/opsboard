use leptos::ev::{MouseEvent, SubmitEvent};
use leptos::prelude::*;
use leptos::{component, event_target_value, view, IntoView};

#[component]
pub fn QuickActions<OnSubmit, OnRunQueue, OnRefresh>(
    job_name: ReadSignal<String>,
    set_job_name: WriteSignal<String>,
    on_submit: OnSubmit,
    on_run_queue: OnRunQueue,
    on_refresh: OnRefresh,
    job_created: ReadSignal<Option<String>>,
    run_result: ReadSignal<Option<String>>,
) -> impl IntoView
where
    OnSubmit: Fn(SubmitEvent) + Copy + 'static,
    OnRunQueue: Fn(MouseEvent) + Copy + 'static,
    OnRefresh: Fn(MouseEvent) + Copy + 'static,
{
    view! {
        <section class="rounded-[22px] border border-slate-200/70 bg-white/90 p-6 shadow-[0_24px_60px_rgba(15,23,42,0.08)] backdrop-blur-sm md:p-7">
            <div class="flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between">
                <form class="flex flex-1 flex-col gap-3 md:flex-row md:flex-wrap md:items-end" on:submit=on_submit>
                    <div class="flex min-w-[240px] flex-1 flex-col gap-2">
                        <label class="text-xs font-bold uppercase tracking-[0.08em] text-slate-500" for="job-name">
                            "Quick actions"
                        </label>
                        
                        <input
                            id="job-name"
                            class="w-full rounded-2xl border border-slate-300 bg-white px-4 py-3.5 text-[15px] text-slate-900 outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-100"
                            type="text"
                            placeholder="Enter job name"
                            prop:value=move || job_name.get()
                        on:input=move |ev| {
                            set_job_name.set(event_target_value(&ev));
                        }
                    />
                    </div>

                    <div class="flex flex-col gap-3 sm:flex-row sm:flex-wrap">
                        <button
                            class="rounded-2xl bg-gradient-to-br from-blue-600 to-blue-700 px-5 py-3 text-sm font-bold text-white shadow-[0_16px_30px_rgba(37,99,235,0.22)] transition hover:-translate-y-0.5"
                            type="submit"
                        >
                            "Create Job"
                        </button>
                        <button
                            class="rounded-2xl bg-slate-200 px-5 py-3 text-sm font-bold text-slate-800 transition hover:-translate-y-0.5"
                            type="button"
                            on:click=on_run_queue
                        >
                            "Run Queue"
                        </button>
                        <button
                            class="rounded-2xl border border-slate-200 bg-slate-50 px-5 py-3 text-sm font-bold text-slate-800 transition hover:-translate-y-0.5"
                            type="button"
                            on:click=on_refresh
                        >
                            "Refresh"
                        </button>
                    </div>
                </form>

                <div class="flex flex-wrap gap-x-4 gap-y-2 text-sm text-slate-600">
                    {move || job_created.get().map(|text| view! { <span class="font-medium text-emerald-700">{text}</span> })}
                    {move || run_result.get().map(|text| view! { <span class="font-medium text-slate-700">{text}</span> })}
                </div>
            </div>
        </section>
    }
}

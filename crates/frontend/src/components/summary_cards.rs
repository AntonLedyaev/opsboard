use leptos::prelude::{Memo, SignalGet};
use leptos::{component, view, IntoView};

use crate::api::JobView;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct JobSummary {
    pub total: usize,
    pub queued: usize,
    pub running: usize,
    pub done: usize,
    pub failed: usize,
}

impl JobSummary {
    pub fn from_jobs(jobs: &[JobView]) -> Self {
        let mut summary = Self {
            total: jobs.len(),
            queued: 0,
            running: 0,
            done: 0,
            failed: 0,
        };

        for job in jobs {
            match job.status.as_str() {
                "Queued" => summary.queued += 1,
                "Running" => summary.running += 1,
                "Done" => summary.done += 1,
                "Failed" => summary.failed += 1,
                _ => {}
            }
        }

        summary
    }
}

#[component]
pub fn SummaryCards(summary: Memo<JobSummary>) -> impl IntoView {
    view! {
        <section class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-5">
            <article class="rounded-[22px] border border-slate-200/70 bg-white/90 p-5 shadow-[0_24px_60px_rgba(15,23,42,0.08)] backdrop-blur-sm">
                <p class="m-0 text-[13px] font-bold uppercase tracking-[0.08em] text-slate-500">"Total Jobs"</p>
                <p class="mt-2 text-4xl font-black text-slate-900">{move || summary.get().total}</p>
                <p class="mt-2 text-sm text-slate-500">"All jobs currently visible in the queue"</p>
            </article>
            <article class="rounded-[22px] border border-amber-200/70 bg-amber-50/80 p-5 shadow-[0_24px_60px_rgba(15,23,42,0.05)]">
                <p class="m-0 text-[13px] font-bold uppercase tracking-[0.08em] text-amber-700">"Queued"</p>
                <p class="mt-2 text-4xl font-black text-amber-950">{move || summary.get().queued}</p>
                <p class="mt-2 text-sm text-amber-800/80">"Waiting to be picked up by the runner"</p>
            </article>
            <article class="rounded-[22px] border border-blue-200/70 bg-blue-50/80 p-5 shadow-[0_24px_60px_rgba(15,23,42,0.05)]">
                <p class="m-0 text-[13px] font-bold uppercase tracking-[0.08em] text-blue-700">"Running"</p>
                <p class="mt-2 text-4xl font-black text-blue-950">{move || summary.get().running}</p>
                <p class="mt-2 text-sm text-blue-800/80">"Currently executing jobs"</p>
            </article>
            <article class="rounded-[22px] border border-emerald-200/70 bg-emerald-50/80 p-5 shadow-[0_24px_60px_rgba(15,23,42,0.05)]">
                <p class="m-0 text-[13px] font-bold uppercase tracking-[0.08em] text-emerald-700">"Done"</p>
                <p class="mt-2 text-4xl font-black text-emerald-950">{move || summary.get().done}</p>
                <p class="mt-2 text-sm text-emerald-800/80">"Completed successfully"</p>
            </article>
            <article class="rounded-[22px] border border-rose-200/70 bg-rose-50/80 p-5 shadow-[0_24px_60px_rgba(15,23,42,0.05)]">
                <p class="m-0 text-[13px] font-bold uppercase tracking-[0.08em] text-rose-700">"Failed"</p>
                <p class="mt-2 text-4xl font-black text-rose-950">{move || summary.get().failed}</p>
                <p class="mt-2 text-sm text-rose-800/80">"Reached retry limit or terminated"</p>
            </article>
        </section>
    }
}

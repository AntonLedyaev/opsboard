use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn HeaderSection(
    loading: ReadSignal<bool>,
    error: ReadSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <section class="rounded-[22px] border border-slate-200/70 bg-white/90 p-7 shadow-[0_24px_60px_rgba(15,23,42,0.08)] backdrop-blur-sm">
            <div class="flex flex-col gap-5 lg:flex-row lg:items-start lg:justify-between">
                <div class="flex flex-col gap-2">
                    <p class="m-0 text-xs font-bold uppercase tracking-[0.18em] text-sky-600">"Dashboard"</p>
                    <h1 class="m-0 text-4xl font-black leading-none text-slate-900 md:text-5xl">"OpsBoard"</h1>
                    <p class="m-0 text-sm text-slate-600 md:text-[15px]">"Mini job orchestration dashboard"</p>
                </div>

                <div class="flex flex-wrap gap-x-4 gap-y-2 text-sm text-slate-600">
                    <span>{move || if loading.get() { "Syncing jobs..." } else { "Auto-refresh every 3 seconds" }}</span>
                    {move || error.get().map(|message| view! { <span class="font-medium text-rose-600">{message}</span> })}
                </div>
            </div>
        </section>
    }
}

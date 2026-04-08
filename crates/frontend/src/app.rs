use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};
use crate::pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
use crate::client::components::{AppError, ErrorList};
use crate::client::pages::user::UserPage;
use leptos::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rust.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorList outside_errors/>
            }
            .into_view()
        }>
            <main class="flex flex-col w-[1000px] mx-auto my-0 py-32">
                <Routes>
                    <Route path="" view=|| view! {  <UserPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

use crate::client::components::error::{AppError, ErrorList};
use crate::client::pages::user::UserListPage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/rust.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorList outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <UserListPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

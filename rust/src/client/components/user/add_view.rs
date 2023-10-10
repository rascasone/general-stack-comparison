use crate::client::components::user::{DetailView, Kind};
use leptos::*;

#[component]
pub fn AddView() -> impl IntoView {
    view! {
        <DetailView kind=Kind::Add />
    }
}

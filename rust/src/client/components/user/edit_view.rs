use crate::client::components::user::{DetailView, Kind};
use leptos::*;

#[component]
pub fn EditView() -> impl IntoView {
    view! {
        <DetailView kind=Kind::Edit />
    }
}

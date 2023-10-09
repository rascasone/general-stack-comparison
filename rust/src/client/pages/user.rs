use crate::client::components::user::AddView;
use crate::client::components::user::EditView;
use crate::client::components::user::ListView;
use crate::client::states::user::{UserPageState, UserPageView};
use leptos::*;

#[component]
pub fn UserPage() -> impl IntoView {
    provide_context(UserPageState::new());

    let state = use_context::<UserPageState>().unwrap();

    move || {
        state.view.with(move |view| match view {
            UserPageView::List => view! { <ListView /> },
            UserPageView::Add => view! { <AddView /> },
            UserPageView::Edit(_) => view! { <EditView /> },
        })
    }
}

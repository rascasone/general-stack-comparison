use crate::client::components::user::{TableNone, TableSome};
use crate::client::states::user::{UserPageState, UserPageView};
use crate::server::user::get_users;
use leptos::*;

#[component]
pub fn ListView() -> impl IntoView {
    let state = use_context::<UserPageState>().unwrap();
    let users_resource = create_local_resource(|| (), |_| get_users());
    let new_user = move |_| state.view.set(UserPageView::Add);

    create_effect(move |_| {
        state
            .users
            .set(users_resource.get().map(|users| users.unwrap()));
    });

    view! {
        <div class="flex flex-row mb-8">
            <h1 class="mr-auto text-4xl font-bold">
                Users
            </h1>

            <button
                class="rounded-md bg-sky-500 text-white px-3 py-1"
                on:click=new_user
            >
                New user
            </button>
        </div>

        {move || {
            state.users.with(move |users| match users {
                None => view! { <TableNone /> },
                Some(_) => view! { <TableSome /> },
            })
        }}
    }
}

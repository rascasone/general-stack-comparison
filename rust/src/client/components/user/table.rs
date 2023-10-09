use crate::client::components::table::TableHead;
use crate::client::states::user::{UserPageState, UserPageView};
use leptos::*;
use rand::{thread_rng, Rng};

#[component]
fn UserTableHead() -> impl IntoView {
    view! {
        <TableHead columns={vec![
            (50, "email"),
            (25, "first name"),
            (25, "last name")
        ]} />
    }
}

#[component]
pub fn TableNone() -> impl IntoView {
    let mut rng = thread_rng();

    view! {
        <table class="bg-white border border-slate-200 shadow-lg rounded-md border-separate table-fixed border-spacing-8 w-full text-left">
            <UserTableHead />

            <tbody class="animate-pulse text-transparent select-none">
                {(0..5).map(|_| view! {
                    <tr>
                        {(0..3).map(|_| view! {
                            <td class="truncate">
                                <div
                                    class="bg-slate-100 rounded-full dark:bg-slate-200"
                                    style=format!("width: {}%", rng.gen_range(25..100))
                                >
                                    .
                                </div>
                            </td>
                        }).collect_view()}
                    </tr>
                }).collect_view()}
            </tbody>
        </table>
    }
}

#[component]
pub fn TableSome() -> impl IntoView {
    let state = use_context::<UserPageState>().unwrap();

    move || {
        state.users.with(move |users| view! {
            <table class="bg-white border border-slate-200 shadow-lg rounded-md border-separate table-fixed border-spacing-8 w-full text-left">
                <UserTableHead />

                <tbody>
                    // TODO: add table empty state
                    {users.as_ref().unwrap().iter().enumerate().map(move |(i, user)| view! {
                        <tr>
                            <td
                                class="truncate cursor-pointer font-bold hover:text-sky-500"
                                on:click=move |_| state.view.set(UserPageView::Edit(i))
                            >
                                <div>{&user.email}</div>
                            </td>

                            <td class="truncate">
                                <div>
                                    {match &user.first_name {
                                        Some(first_name) => first_name.clone(),
                                        None => "-".to_string(),
                                    }}
                                </div>
                            </td>

                            <td class="truncate">
                                <div>
                                    {match &user.last_name {
                                        Some(last_name) => last_name.clone(),
                                        None => "-".to_string(),
                                    }}
                                </div>
                            </td>
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        })
    }
}

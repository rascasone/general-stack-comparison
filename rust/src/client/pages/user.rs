use leptos::*;

use crate::server::user::get_all_users;

#[component]
pub fn UserListPage(cx: Scope) -> impl IntoView {
    let users = create_resource(cx, || (), |_| get_all_users());

    view! {
        cx,
        <div class="p-16 flex flex-col w-[750px] my-0 mx-auto">
            <button class="border border-slate-700 px-3 py-2 rounded-md mb-4 w-fit">
                Add new user
            </button>

            <Suspense fallback=move || view! { cx, <p>"Loading .."</p> }>
                {move || users.with(cx, |users| match users {
                    Err(e) => view! { cx, <pre>{e.to_string()}</pre> }.into_view(cx),
                    Ok(users) => view! {
                        cx,
                        <table class="table-auto border-collapse w-full">
                            <tr>
                                <th class="text-left border border-slate-700 px-3 py-2">Id</th>
                                <th class="text-left border border-slate-700 px-3 py-2">Email</th>
                            </tr>
                            {users
                                .iter()
                                .map(move |user| view! {
                                    cx,
                                    <tr>
                                        <td class="text-left border border-slate-700 px-3 py-2">{&user.id}</td>
                                        <td class="text-left border border-slate-700 px-3 py-2">{&user.email}</td>
                                    </tr>
                                }.into_view(cx))
                                .collect_view(cx)}
                        </table>
                    }.into_view(cx)
                })}
            </Suspense>
        </div>
    }
}

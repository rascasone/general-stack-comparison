use crate::client::components::user::{DetailView, EditableUser, Kind};
use crate::client::states::user::{UserPageState, UserPageView};
use crate::models::NewUserProps;
use crate::server::user::create_user;
use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn AddView() -> impl IntoView {
    let state = use_context::<UserPageState>().unwrap();
    let close = move |_: MouseEvent| state.view.set(UserPageView::List);
    let user = EditableUser::default();

    let save = move |ev: MouseEvent| {
        spawn_local(async move {
            create_user(NewUserProps {
                email: user.email.get_untracked(),
                first_name: user.first_name.get_untracked(),
                last_name: user.last_name.get_untracked(),
                gender: user.gender.get_untracked(),
                education: user.education.get_untracked(),
                // birth_date: user.birth_date.get_untracked(),
            })
            .await
            .expect("Couldn't create user");

            close(ev);
        });
    };

    view! {
        <DetailView
            kind=Kind::Add
            user=user
            on_save=save
            on_close=close
            on_delete=move |_| {}
        />
    }
}

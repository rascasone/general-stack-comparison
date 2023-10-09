use crate::client::components::user::{DetailView, EditableUser, Kind};
use crate::client::states::user::{UserPageState, UserPageView};
use crate::models::ChangesetUser;
use crate::server::user::{delete_user, update_user};
use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn EditView() -> impl IntoView {
    let state = use_context::<UserPageState>().unwrap();
    let close = move |_: MouseEvent| state.view.set(UserPageView::List);
    let user = match state.view.get() {
        UserPageView::Edit(index) => {
            let indexed_user = &state.users.get_untracked().unwrap()[index];

            EditableUser {
                _id: RwSignal::new(indexed_user.id.clone()),
                email: RwSignal::new(indexed_user.email.clone()),
                first_name: RwSignal::new(
                    indexed_user.first_name.clone().unwrap_or("".to_string()),
                ),
                last_name: RwSignal::new(indexed_user.last_name.clone().unwrap_or("".to_string())),
                gender: RwSignal::new(indexed_user.gender.clone().unwrap_or("".to_string())),
                education: RwSignal::new(indexed_user.education.clone().unwrap_or("".to_string())),
                // birth_date: RwSignal::new(match indexed_user.birth_date {
                //     Some(birth_date) => birth_date.to_string(),
                //     None => "".to_string(),
                // }),
                // valid: RwSignal::new(indexed_user.valid.clone().unwrap_or(false).to_string()),
            }
        }
        _ => unreachable!(),
    };

    let save = move |ev: MouseEvent| {
        spawn_local(async move {
            update_user(
                user.email.get_untracked(),
                ChangesetUser {
                    email: match user.email.get_untracked() {
                        email if email.is_empty() => None,
                        email => Some(email),
                    },
                    first_name: match user.first_name.get_untracked() {
                        first_name if first_name.is_empty() => None,
                        first_name => Some(first_name),
                    },
                    last_name: match user.last_name.get_untracked() {
                        last_name if last_name.is_empty() => None,
                        last_name => Some(last_name),
                    },
                    gender: match user.gender.get_untracked() {
                        gender if gender.is_empty() => None,
                        gender => Some(gender),
                    },
                    education: match user.education.get_untracked() {
                        education if education.is_empty() => None,
                        education => Some(education),
                    },
                    // birth_date: user.birth_date.get_untracked(),
                    // valid: user.valid.get_untracked(),
                },
            )
            .await
            .expect("Couldn't create user");

            close(ev);
        });
    };

    let delete = move |ev: MouseEvent| {
        spawn_local(async move {
            delete_user(user._id.get_untracked())
                .await
                .expect("Couldn't delete user");

            close(ev);
        });
    };

    view! {
        <DetailView
            kind=Kind::Edit
            user=user
            on_save=save
            on_close=close
            on_delete=delete
        />
    }
}

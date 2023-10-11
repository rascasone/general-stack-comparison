use crate::client::components::{Check, Field, Form, Props};
use crate::client::states::user::{UserPageState, UserPageView};
use crate::models::{NewUserProps, UpdateUser};
use crate::server::user::{create_user, delete_user, update_user};
use leptos::ev::MouseEvent;
use leptos::*;

pub struct EditableUser {
    pub id: RwSignal<String>,
    pub email: RwSignal<String>,
    pub first_name: RwSignal<String>,
    pub last_name: RwSignal<String>,
    pub gender: RwSignal<String>,
    pub education: RwSignal<String>,
    pub birth_date: RwSignal<String>,
    pub valid: RwSignal<String>,
}

impl Default for EditableUser {
    fn default() -> Self {
        Self {
            id: RwSignal::new("".to_string()),
            email: RwSignal::new("".to_string()),
            first_name: RwSignal::new("".to_string()),
            last_name: RwSignal::new("".to_string()),
            gender: RwSignal::new("".to_string()),
            education: RwSignal::new("".to_string()),
            birth_date: RwSignal::new("".to_string()),
            valid: RwSignal::new("".to_string()),
        }
    }
}

pub enum Kind {
    Add,
    Edit,
}

#[component]
pub fn DetailView(kind: Kind) -> impl IntoView {
    let state = use_context::<UserPageState>().unwrap();
    let close = move |_: MouseEvent| state.view.set(UserPageView::List);
    let error_counter = RwSignal::<usize>::new(0);

    let is_add = match &kind {
        Kind::Add => true,
        Kind::Edit => false,
    };

    let user = match state.view.get() {
        UserPageView::Edit(index) => {
            let indexed_user = &state.users.get_untracked().unwrap()[index];

            EditableUser {
                id: RwSignal::new(indexed_user.id.clone()),
                email: RwSignal::new(indexed_user.email.clone()),
                first_name: RwSignal::new(
                    indexed_user.first_name.clone().unwrap_or("".to_string()),
                ),
                last_name: RwSignal::new(indexed_user.last_name.clone().unwrap_or("".to_string())),
                gender: RwSignal::new(indexed_user.gender.clone().unwrap_or("".to_string())),
                education: RwSignal::new(indexed_user.education.clone().unwrap_or("".to_string())),
                birth_date: RwSignal::new(match indexed_user.birth_date {
                    Some(birth_date) => birth_date.to_string(),
                    None => "".to_string(),
                }),
                valid: RwSignal::new(indexed_user.valid.unwrap_or(false).to_string()),
            }
        }
        _ => EditableUser::default(),
    };

    let save = move |ev: MouseEvent| {
        spawn_local(async move {
            if is_add {
                create_user(NewUserProps {
                    email: user.email.get_untracked(),
                    first_name: user.first_name.get_untracked(),
                    last_name: user.last_name.get_untracked(),
                    gender: user.gender.get_untracked(),
                    education: user.education.get_untracked(),
                    birth_date: user.birth_date.get_untracked(),
                })
                .await
                .expect("Couldn't create user");
            } else {
                update_user(
                    user.id.get_untracked(),
                    UpdateUser {
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
                        birth_date: match user.birth_date.get_untracked() {
                            birth_date if birth_date.is_empty() => None,
                            birth_date => Some(birth_date),
                        },
                        valid: match user.valid.get_untracked() {
                            valid if valid.is_empty() => None,
                            valid => Some(valid),
                        },
                    },
                )
                .await
                .expect("Couldn't create user");
            }

            close(ev);
        });
    };

    let delete = move |ev: MouseEvent| {
        spawn_local(async move {
            delete_user(user.id.get_untracked())
                .await
                .expect("Couldn't delete user");

            close(ev);
        });
    };

    view! {
         <div class="flex flex-row mb-8">
             <h1 class="mr-auto text-4xl font-bold">
                {match &kind {
                    Kind::Add => "New user".to_string(),
                    Kind::Edit => user.email.get(),
                }}
             </h1>

            <Show when=move || !is_add>
                 <button
                     class="rounded-md bg-slate-200 text-black px-3 py-1 mr-2"
                     on:click=delete
                 >
                    Delete
                 </button>
            </Show>

             <button
                 class="rounded-md bg-slate-200 text-black px-3 py-1"
                 on:click=close
             >
                 Close
             </button>

             <button
                 class="rounded-md bg-sky-500 text-white px-3 py-1 ml-2 disabled:opacity-50"
                 on:click=save
                 disabled=move || (error_counter.get() > 0)
             >
                {match &kind {
                    Kind::Add => "Add",
                    Kind::Edit => "Save",
                }}
             </button>
         </div>

         <div class="flex flex-col bg-white border border-slate-200 shadow-lg rounded-md w-full text-left p-8">
            <Form
                error_counter=error_counter
                fields=vec![
                    Field::Input(
                        "email",
                        Props {
                            label: "Email",
                            value: user.email,
                            checks: vec![Check::required, Check::email],
                        },
                    ),
                    Field::Input(
                        "text",
                        Props {
                            label: "First name",
                            value: user.first_name,
                            checks: vec![Check::word],
                        },
                    ),
                    Field::Input(
                        "text",
                        Props {
                            label: "Last name",
                            value: user.last_name,
                            checks: vec![Check::word],
                        },
                    ),
                    Field::Input(
                        "text",
                        Props {
                            label: "Gender",
                            value: user.gender,
                            checks: vec![],
                        },
                    ),
                    Field::Input(
                        "text",
                        Props {
                            label: "Education",
                            value: user.education,
                            checks: vec![],
                        },
                    ),
                    Field::Date(
                        (1900, 2023),
                        Props {
                            label: "Birth date",
                            value: user.birth_date,
                            checks: vec![],
                        },
                    ),
                    Field::Checkbox(
                        Props {
                            label: "Valid",
                            value: user.valid,
                            checks: vec![],
                        },
                    ),
                ]
            />
         </div>
    }
}

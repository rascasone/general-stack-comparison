use crate::client::components::{Form, Input};
use leptos::ev::MouseEvent;
use leptos::*;

pub struct EditableUser {
    pub _id: RwSignal<String>,
    pub email: RwSignal<String>,
    pub first_name: RwSignal<String>,
    pub last_name: RwSignal<String>,
    pub gender: RwSignal<String>,
    pub education: RwSignal<String>,
    // pub birth_date: RwSignal<String>,
    // pub valid: RwSignal<String>,
}

impl Default for EditableUser {
    fn default() -> Self {
        Self {
            _id: RwSignal::new("".to_string()),
            email: RwSignal::new("".to_string()),
            first_name: RwSignal::new("".to_string()),
            last_name: RwSignal::new("".to_string()),
            gender: RwSignal::new("".to_string()),
            education: RwSignal::new("".to_string()),
            // birth_date: RwSignal::new("".to_string()),
            // valid: RwSignal::new("".to_string()),
        }
    }
}

pub enum Kind {
    Add,
    Edit,
}

#[component]
pub fn DetailView<OnSave, OnClose, OnDelete>(
    kind: Kind,
    user: EditableUser,
    // TODO: do saving, closing and deleting here
    on_save: OnSave,
    on_close: OnClose,
    on_delete: OnDelete,
) -> impl IntoView
where
    OnSave: Fn(MouseEvent) + 'static,
    OnClose: Fn(MouseEvent) + 'static,
    OnDelete: Fn(MouseEvent) + 'static,
{
    let error_counter = RwSignal::<usize>::new(0);
    // let is_add = match &kind {
    //     Kind::Add => false,
    //     Kind::Edit => true,
    // };

    view! {
         <div class="flex flex-row mb-8">
             <h1 class="mr-auto text-4xl font-bold">
                {match &kind {
                    Kind::Add => "New user".to_string(),
                    Kind::Edit => user.email.get(),
                }}
             </h1>

             <button
                 class="rounded-md bg-slate-200 text-black px-3 py-1 mr-2"
                 on:click=on_delete
             >
                Delete
             </button>

             <button
                 class="rounded-md bg-slate-200 text-black px-3 py-1"
                 on:click=on_close
             >
                 Close
             </button>

             <button
                 class="rounded-md bg-sky-500 text-white px-3 py-1 ml-2"
                 on:click=on_save
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
                inputs=vec![
                    Input::email("Email", user.email),
                    Input::text("First name", user.first_name),
                    Input::text("Last name", user.last_name),
                    Input::text("Gender", user.gender),
                    Input::text("Education", user.education),
                    // Input::text("Birth date", user.birth_date),
                ]
                error_counter=error_counter
            />
         </div>
    }
}

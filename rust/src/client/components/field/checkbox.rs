use leptos::ev::MouseEvent;
use leptos::*;

use crate::client::components::field::{use_field, Error, Label, Wrapper};
use crate::client::components::form::Props;

#[component]
fn Button(
    active: Memo<bool>,
    label: &'static str,
    on_click: Callback<MouseEvent>,
) -> impl IntoView {
    move || match active.get() {
        true => view! {
            <button
                class="mr-2 last:mr-0 rounded-md border border-slate-200 px-3 py-1 text-black bg-sky-500 text-white"
                on:click=on_click
            >
                {label}
            </button>
        },
        false => view! {
            <button
                class="mr-2 last:mr-0 rounded-md border border-slate-200 px-3 py-1 text-black hover:bg-sky-500 hover:text-white"
                on:click=on_click
            >
                {label}
            </button>
        },
    }
}

#[component]
pub fn Checkbox(error_counter: RwSignal<usize>, props: Props) -> impl IntoView {
    let field = use_field(props.value, error_counter, props.checks);
    let true_active = create_memo(move |_| matches!(field.value.get().as_str(), "true"));
    let false_active = create_memo(move |_| matches!(field.value.get().as_str(), "false"));

    view! {
        <Wrapper>
            <Label required=field.required label=props.label />

            <div class="w-fit p-2 flex flex-row rounded-md border border-slate-200">
                <Button
                    active=true_active
                    label="Yes"
                    on_click=Callback::from(move |_| field.value.set("true".to_owned()))
                />
                <Button
                    active=false_active
                    label="No"
                    on_click=Callback::from(move |_| field.value.set("false".to_owned()))
                />
            </div>

            <Error error=field.error />
        </Wrapper>
    }
}

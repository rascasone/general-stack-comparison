use crate::client::components::Input as InputComponent;
use crate::client::components::Spacer;
use leptos::*;

pub struct Input {
    pub kind: &'static str,
    pub required: bool,
    pub label: &'static str,
    pub value: RwSignal<String>,
}

impl Input {
    pub fn text(label: &'static str, value: RwSignal<String>) -> Self {
        Self {
            kind: "text",
            required: false,
            label,
            value,
        }
    }
    pub fn email(label: &'static str, value: RwSignal<String>) -> Self {
        Self {
            kind: "email",
            required: true,
            label,
            value,
        }
    }
}

#[component]
pub fn Form(inputs: Vec<Input>, error_counter: RwSignal<usize>) -> impl IntoView {
    let check_trigger = create_trigger();
    let input_length = inputs.len() - 1;

    view! {
        <form>
            {inputs.iter().enumerate().map(|(i, input)| view! {
                <InputComponent
                    label=input.label
                    kind=input.kind
                    required=input.required
                    value=input.value
                    error_counter=error_counter
                    check_trigger=check_trigger
                />
                <Show when=move || i != input_length>
                    <Spacer />
                </Show>
            }).collect_view()}
        </form>
    }
}

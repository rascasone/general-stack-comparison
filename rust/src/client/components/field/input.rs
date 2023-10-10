use leptos::*;

use crate::client::components::field::{use_field, Error, Label, Wrapper};
use crate::client::components::form::Props;

#[component]
pub fn Input(kind: &'static str, error_counter: RwSignal<usize>, props: Props) -> impl IntoView {
    let field = use_field(props.value, error_counter, props.checks);

    view! {
        <Wrapper>
            <Label required=field.required label=props.label />

            <input
                class="w-[300px] py-2 px-4 border border-slate-200 rounded-md"
                type=kind
                prop:value=field.value
                on:input=move |ev| (field.on_change)(event_target_value(&ev))
                required=field.required
            />

            <Error error=field.error />
        </Wrapper>
    }
}

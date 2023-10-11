use leptos::*;

use crate::client::components::field::{Checkbox, Date, Input};
use crate::client::components::Spacer;

pub enum Field {
    Input(&'static str, Props),
    Checkbox(Props),
    Date((u32, u32), Props),
}

#[derive(Clone)]
pub struct Props {
    pub label: &'static str,
    pub value: RwSignal<String>,
    pub checks: Vec<fn(&str) -> Option<String>>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: "",
            value: RwSignal::new(String::default()),
            checks: vec![],
        }
    }
}

pub struct Check {}

impl Check {
    pub fn required(v: &str) -> Option<String> {
        if v.is_empty() {
            Some("This field is required".to_owned())
        } else {
            None
        }
    }

    pub fn email(v: &str) -> Option<String> {
        if !v.contains('@') {
            Some("Invalid email format".to_owned())
        } else {
            None
        }
    }

    pub fn word(v: &str) -> Option<String> {
        if v.contains(' ') {
            Some("Should be a single word".to_owned())
        } else {
            None
        }
    }
}

#[component]
pub fn Form(fields: Vec<Field>, error_counter: RwSignal<usize>) -> impl IntoView {
    let input_length = fields.len() - 1;

    view! {
        <div>
            {fields.iter().enumerate().map(|(i, field)| view! {
                {match field {
                    Field::Input(kind, props) => view! {
                        <Input
                            props=props.clone()
                            kind=kind
                            error_counter=error_counter
                        />
                    },
                    Field::Checkbox(props) => view! {
                        <Checkbox
                            props=props.clone()
                            error_counter=error_counter
                        />
                    },
                    Field::Date(range, props) => view! {
                        <Date
                            range=*range
                            props=props.clone()
                            error_counter=error_counter
                        />
                    },
                }}

                <Show when=move || i != input_length>
                    <Spacer />
                </Show>
            }).collect_view()}
        </div>
    }
}

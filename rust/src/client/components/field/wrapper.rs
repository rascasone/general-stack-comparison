use leptos::logging::log;
use leptos::*;

use crate::client::components::Check;

pub struct UseField {
    pub value: RwSignal<String>,
    pub required: bool,
    pub error: RwSignal<Option<String>>,
    pub on_change: Box<dyn Fn(String)>,
}

pub fn use_field(
    value: RwSignal<String>,
    error_counter: RwSignal<usize>,
    checks: Vec<fn(&str) -> Option<String>>,
) -> UseField {
    let error = RwSignal::<Option<String>>::new(None);
    let required = checks
        .iter()
        .any(|check| *check as usize == Check::required as usize);

    let run_checks = move |v: &String| {
        for check in &checks {
            let check_error = check(v);

            if check_error.is_some() {
                if error.get_untracked().is_none() {
                    error_counter.update(|v| *v = *v + 1);
                }

                error.set(check_error);
                return false;
            }
        }

        true
    };

    run_checks(&value.get_untracked());

    let on_change = move |v: String| {
        if run_checks(&v) {
            if error.get_untracked().is_some() {
                error_counter.update(|v| *v = *v - 1);
            }

            error.set(None);
        }

        value.set(v);
    };

    create_effect(move |_| {
        log!(
            "error: {}",
            match error.get() {
                Some(e) => e,
                None => "None".to_owned(),
            }
        );
    });

    UseField {
        value,
        required,
        error,
        on_change: Box::new(on_change),
    }
}

#[component]
pub fn Wrapper(children: Children) -> impl IntoView {
    view! {
         <div class="flex flex-col">
            {children()}
        </div>
    }
}

#[component]
pub fn Label(required: bool, label: &'static str) -> impl IntoView {
    view! {
        <Show when=move || !label.is_empty()>
            <label class="mb-2 text-sm text-slate-400 font-bold">
                {match required {
                    true => format!("* {}", label),
                    false => label.to_owned(),
                }}
            </label>
        </Show>
    }
}

#[component]
pub fn Error(error: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        <Show when=move || error.get().is_some()>
            <div class="text-red-500 text-sm mt-1">
                {error.get().unwrap()}
            </div>
        </Show>
    }
}

use leptos::*;

#[component]
pub fn Input(
    value: RwSignal<String>,
    error_counter: RwSignal<usize>,
    check_trigger: Trigger,
    #[prop(optional)] label: &'static str,
    #[prop(optional, default = "text")] kind: &'static str,
    #[prop(optional, default = false)] required: bool,
) -> impl IntoView {
    let show_error = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    let run_checks = move |value: String, inner: bool| {
        if inner {
            show_error.set(true);
        }

        if required && value.is_empty() {
            error.set(Some("This field is required".to_string()));
            error_counter.update(|v| *v = *v + 1);
            return;
        }

        if error.get().is_some() {
            error_counter.update(|v| *v = *v - 1);
        }

        error.set(None);
    };

    create_effect(move |_| {
        check_trigger.try_track();
        run_checks(value.get_untracked(), false);
    });

    let on_input = move |ev| {
        let target_value = event_target_value(&ev);

        run_checks(target_value.clone(), true);
        value.set(target_value);
    };

    view! {
         <div class="flex flex-col">
            <Show when=move || !label.is_empty()>
                <label class="mb-2 text-sm text-slate-400 font-bold">
                    {match required {
                        true => format!("* {}", label),
                        false => label.to_string(),
                    }}
                </label>
            </Show>

            <input
                class="w-[300px] py-2 px-4 border border-slate-200 rounded-md"
                type=kind
                prop:value=value
                on:input=on_input
            />

            <Show when=move || show_error.get() && error.get().is_some()>
                <div class="text-red-500 text-sm mt-1">
                    {error.get_untracked().unwrap()}
                </div>
            </Show>
         </div>
    }
}

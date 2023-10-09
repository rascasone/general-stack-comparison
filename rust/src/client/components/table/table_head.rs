use leptos::*;

#[component]
pub fn TableHead(columns: Vec<(usize, &'static str)>) -> impl IntoView {
    view! {
        <colgroup>
            {columns.iter().map(|(width, _)| view! {
                <col style=format!("width: {}%", width) />
            }).collect_view()}
        </colgroup>

        <thead class="text-sm text-slate-400">
            <tr>
                {columns.iter().map(|(_, name)| view! {
                    <th>{name.to_uppercase()}</th>
                }).collect_view()}
            </tr>
        </thead>
    }
}

use cfg_if::cfg_if;

pub mod client;
pub mod database;
pub mod models;
pub mod schema;
pub mod server;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::client::app::App;

    #[wasm_bindgen]
    pub fn hydrate() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || {
            view! {  <App/> }
        });
    }
}}

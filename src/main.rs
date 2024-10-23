mod app;
mod state;
mod auth;
mod consts;
mod utils;
mod canister;
mod components;
mod error_template;
mod base_route;
mod models;
use app::*;
use leptos::*;

pub fn main() {
    // dotenv::dotenv().ok();
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App /> }
    });
}
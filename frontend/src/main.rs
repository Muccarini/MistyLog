mod api;
mod app;
mod components;
mod pages;
mod shadcn_classes;
mod mock_data;

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    leptos::mount::mount_to_body(app::App);
}

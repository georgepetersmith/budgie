use leptos::mount::mount_to_body;

mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(components::App)
}
use budgie::Budget;
use js_sys::wasm_bindgen::JsCast;
use leptos::prelude::*;
use leptos::{ev::Event, html::Input};
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[component]
pub fn BudgetImport() -> impl IntoView {
    let budget = use_context::<RwSignal<Budget>>().unwrap();
    let file_input: NodeRef<Input> = NodeRef::new();

    let on_file_change = move |ev: Event| {
        let input = ev.target().unwrap();
        let input = input.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        budget.set(Budget::default());

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                spawn_local(async move {
                    match read_file_as_text(&file).await {
                        Ok(text) => match serde_json::from_str::<Budget>(&text) {
                            Ok(data) => budget.set(data),
                            Err(e) => window()
                                .unwrap()
                                .alert_with_message(&format!("Parse error: {}", e))
                                .unwrap(),
                        },
                        Err(e) => window().unwrap().alert_with_message(&e).unwrap(),
                    }
                });
            }
        }
    };

    async fn read_file_as_text(file: &web_sys::File) -> Result<String, String> {
        let array_buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
            .await
            .map_err(|_| "Failed to read file".to_string())?;

        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();

        String::from_utf8(bytes).map_err(|_| "Invalid UTF-8".to_string())
    }
    view! {
        <input
            type="file"
            id="import-file"
            name="import-file"
            accept=".json"
            node_ref=file_input
            on:change=on_file_change
        />
    }
}

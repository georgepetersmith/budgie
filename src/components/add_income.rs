use budgie::{Budget, Income};
use leptos::prelude::*;

#[component]
pub fn AddIncome() -> impl IntoView {
    let budget = use_context::<RwSignal<Budget>>().unwrap();
    let (amount, set_amount) = signal(0.0f64);
    view! {
        <div style="display:flex; align-items:center; gap:10px;">
            <input
                type="number"
                name="amount"
                prop:value=amount
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Ok(a) = val.parse::<f64>() {
                        set_amount.set(a);
                    }
                }
            />
            <button
                type="button"
                on:click=move |_| {
                    budget.update(move |b: &mut Budget| b.add_income(Income::new(amount.get())));
                    set_amount.set(0f64);
                }
            >
                "+"
            </button>
        </div>
    }
}


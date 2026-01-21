use budgie::{Account, AccountId, Budget};
use leptos::leptos_dom::logging::console_error;
use leptos::prelude::*;
use rust_decimal::prelude::*;

#[component]
pub fn AddExpenditure() -> impl IntoView {
    let budget = use_context::<RwSignal<Budget>>().unwrap();
    let (name, set_name) = signal(String::new());
    let (amount, set_amount) = signal(0f64);
    let (account, set_account) = signal(AccountId(0));
    view! {
        <div style="display:flex; flex-direction:column; gap:10px;">
            <input
                type="text"
                name="name"
                prop:value=name
                placeholder="e.g. Broadband"
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    set_name.set(val);
                }
            />
            <input
                type="number"
                name="amount"
                placeholder="e.g. 50.99"
                prop:value=amount
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Ok(a) = val.parse::<f64>() {
                        set_amount.set(a);
                    }
                }
            />
            <select
                on:change:target=move |ev| {
                    set_account.set(AccountId(ev.target().value().parse().unwrap()));
                }
                prop:value=move || account.get().0
            >

                <For
                    each=move || budget.with(|b| b.accounts().to_vec())
                    key=|state| state.id
                    let(Account { id, name })
                >
                    <option value=id.0>{name}</option>
                </For>
            </select>
            <button
                style="width:50%; margin-left:auto; background:#22c55e;"
                type="button"
                on:click=move |_| {
                    budget
                        .update(move |b: &mut Budget| {
                            if let Err(e) = b
                                .add_expenditure(
                                    name.get(),
                                    Decimal::from_f64(amount.get()).unwrap(),
                                    account.get(),
                                    None,
                                )
                            {
                                console_error(&e);
                            }
                        });
                    set_name.set(String::new());
                    set_amount.set(0f64);
                }
            >
                "+"
            </button>
        </div>
    }
}

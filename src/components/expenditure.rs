use leptos::prelude::*;
use budgie::{Budget, Expenditure};
use crate::components::add_expenditure::AddExpenditure;

#[component]
pub fn Expenditure() -> impl IntoView {
    let budget = use_context::<RwSignal<Budget>>().expect("Budget context not found");
    view! {
        <div style="padding:10px; border:2px solid red; border-radius:10px;">
            <h3>"Outgoing"</h3>
            <AddExpenditure />
            <div style="display:flex; flex-direction:column; gap:10px;">
                <For
                    each=move || budget.with(|b| b.expenditures().to_vec())
                    key=|state| state.id
                    let(Expenditure { id, name, amount })
                >
                    <div style="margin-top: 10px; display:flex; align-items:center; justify-content:space-between; gap:10px;">
                        <span>{name}</span>
                        <span>{amount}</span>
                        <button
                            type="button"
                            on:click=move |_| {
                                budget
                                    .update(move |b: &mut Budget| {
                                        b.remove_expenditure(id);
                                    });
                            }
                        >
                            "-"
                        </button>
                    </div>
                </For>
            </div>
        </div>
    }
}
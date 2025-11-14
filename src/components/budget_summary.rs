use leptos::prelude::*;
use budgie::Budget;

#[component]
pub fn BudgetSummary() -> impl IntoView {
    let budget = use_context::<RwSignal<Budget>>().expect("Budget context not found");
    
    view! {
        <section style="padding:10px; border:2px solid gray; border-radius:10px;">
            <h3>Summary</h3>
            <p>"Total Income: " {move || budget.get().total_income()}</p>
            <p>"Total Expenditure: " {move || budget.get().total_expenditure()}</p>
            <p>"Remaining: " {move || budget.get().balance()}</p>
        </section>
    }
}

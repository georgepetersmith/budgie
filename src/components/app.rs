use budgie::Budget;
use leptos::prelude::*;

use crate::components::{add_expenditure::AddExpenditure, add_income::AddIncome};

#[component]
pub fn App() -> impl IntoView {
    let budget = RwSignal::new(Budget::default());
    provide_context(budget);
    view! {
        <div style="padding:50px;">
            <section style="padding:10px; border:2px solid gray; border-radius:10px;">
                <h3>Summary</h3>
                <p>"Total Income: " {move || budget.get().total_income()}</p>
                <p>"Total Expenditure: " {move || budget.get().total_expenditure()}</p>
                <p>"Remaining: " {move || budget.get().balance()}</p>
            </section>
            <div style="margin-top: 50px; display:flex; align-items:flex-start; gap:50px;">
                <div style="padding:10px; border:2px solid green; border-radius:10px;">
                    <h3>"Income"</h3>
                    <AddIncome />
                </div>
                <div style="padding:10px; border:2px solid red; border-radius:10px;">
                    <h3>"Expenditure"</h3>
                    <AddExpenditure />
                </div>
            </div>
        </div>
    }
}

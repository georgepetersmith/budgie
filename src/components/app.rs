use budgie::Budget;
use leptos::prelude::*;
use crate::components::budget_summary::BudgetSummary;
use crate::components::expenditure::Expenditure;
use crate::components::income::Income;

#[component]
pub fn App() -> impl IntoView {
    let budget = RwSignal::new(Budget::default());
    provide_context(budget);
    view! {
        <div style="padding:50px;">
            <BudgetSummary />
            <div style="margin-top: 50px; display:flex; align-items:flex-start; gap:50px;">
                <Income />
                <Expenditure />
            </div>
        </div>
    }
}

use crate::components::{budget_import::BudgetImport, ledger::Ledger};
use budgie::Budget;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let budget = RwSignal::new(Budget::new("New Budget".to_string()));
    provide_context(budget);

    view! {
        <div class="app">
            <h1>{move || budget.get().name().to_string()}</h1>
            <BudgetImport />
            <section class="summary">
                <div class="card">
                    <div class="card-title">"TOTAL INCOME"</div>
                    <div class="card-value positive">
                        {move || format!("£{:.2}", budget.get().total_income().abs())}
                    </div>
                </div>
                <div class="card">
                    <div class="card-title">"TOTAL OUTGOING"</div>
                    <div class="card-value negative">
                        {move || format!("£{:.2}", budget.get().total_expenditure().abs())}
                    </div>
                </div>
                <div class="card">
                    <div class="card-title">"CURRENT BALANCE"</div>
                    <div
                        class="card-value"
                        class:positive=move || budget.get().balance().ge(&0f64)
                        class:negative=move || budget.get().balance().lt(&0f64)
                    >
                        {move || format!("£{:.2}", budget.get().balance())}
                    </div>
                </div>
            </section>
            <section>
                <Ledger />
            </section>
        </div>
    }
}

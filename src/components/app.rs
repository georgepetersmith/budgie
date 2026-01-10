use crate::components::{add_expenditure::AddExpenditure, add_income::AddIncome};
use budgie::{Budget, Expenditure, Income};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let budget = {
        let file_str = include_str!("../../my_budget.json");
        let budget: Budget = serde_json::from_str::<Budget>(file_str).expect("valid json");
        budget
    };

    let budget = RwSignal::new(budget);
    provide_context(budget);

    view! {
        <div class="app">
            <h1>My Budget</h1>
            <section class="summary">
                <div class="card" style="flex-grow:1;">
                    <div class="card-title">"TOTAL INCOME"</div>
                    <div class="card-value positive">
                        {move || format!("£{:.2}", budget.get().total_income().abs())}
                    </div>
                </div>
                <div class="card" style="flex-grow:1;">
                    <div class="card-title">"TOTAL OUTGOING"</div>
                    <div class="card-value negative">
                        {move || format!("£{:.2}", budget.get().total_expenditure().abs())}
                    </div>
                </div>
                <div class="card" style="flex-grow:1;">
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
            <section style="display:flex; gap:30px; margin-top: 60px; ">
                <div style="display:flex; flex-direction:column; flex-grow:1; gap:30px;">
                    <div class="card">
                        <div class="section-title">"ADD INCOME"</div>
                        <AddIncome />
                    </div>
                    <div class="card">
                        <div class="section-title">"ADD OUTGOING"</div>
                        <AddExpenditure />
                    </div>
                </div>
                <div style="flex-grow:3;">
                    <div class="card">
                        <div class="section-title">"LEDGER"</div>
                        <div style="table-wrapper">
                            <table>
                                <thead>
                                    <tr>
                                        <th>"Type"</th>
                                        <th>"Description"</th>
                                        <th>"Amount"</th>
                                        <th>"Account"</th>
                                        <th></th>
                                    </tr>
                                </thead>
                                <tbody id="entries-body">
                                    <For
                                        each=move || budget.with(|b| b.incomes().to_vec())
                                        key=|state| state.id
                                        let(Income { id, name, amount, account_id })
                                    >
                                        <tr>
                                            <td class="badge badge-income">"Income"</td>
                                            <td>{name}</td>
                                            <td class="amount-income">
                                                {move || format!("£{:.2}", amount)}
                                            </td>
                                            <td>
                                                {move || {
                                                    budget
                                                        .with(|b| {
                                                            b.get_account(&account_id)
                                                                .expect("account exists")
                                                                .name
                                                                .clone()
                                                        })
                                                }}
                                            </td>
                                            <td>
                                                <button
                                                    type="button"
                                                    on:click=move |_| {
                                                        budget
                                                            .update(move |b: &mut Budget| {
                                                                b.remove_income(id);
                                                            });
                                                    }
                                                >
                                                    "-"
                                                </button>
                                            </td>
                                        </tr>
                                    </For>
                                    <For
                                        each=move || budget.with(|b| b.expenditures().to_vec())
                                        key=|state| state.id
                                        let(Expenditure { id, name, amount, account_id, commitment: _ })
                                    >
                                        <tr>
                                            <td class="badge badge-expense">"Outgoing"</td>
                                            <td>{name}</td>
                                            <td class="amount-expense">
                                                {move || format!("£{:.2}", amount)}
                                            </td>
                                            <td>
                                                {move || {
                                                    budget
                                                        .with(|b| {
                                                            b.get_account(&account_id)
                                                                .expect("account exists")
                                                                .name
                                                                .clone()
                                                        })
                                                }}
                                            </td>
                                            <td>
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
                                            </td>
                                        </tr>
                                    </For>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}

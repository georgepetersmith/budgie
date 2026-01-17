use crate::components::{add_expenditure::AddExpenditure, add_income::AddIncome};
use budgie::{Budget, ExpenditureId, IncomeId};
use leptos::prelude::*;

#[component]
pub fn Ledger() -> impl IntoView {
    let budget = use_context::<RwSignal<Budget>>().expect("Budget exists");

    view! {
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
                <div class="table-wrapper">
                    <table>
                        <thead>
                            <tr>
                                <th>"Type"</th>
                                <th>"Amount"</th>
                                <th>"Description"</th>
                                <th>"Account"</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody id="entries-body">
                            <For
                                each=move || budget.with(|b| b.into_ledger())
                                key=|state| state.description.clone()
                                let(LedgerItem {
                                    item_type,
                                    item_type_string,
                                    amount,
                                    description,
                                    account,
                                })
                            >
                                <tr>
                                    <td
                                        class="badge"
                                        class=(
                                            "badge-income",
                                            move || matches!(item_type, LedgerItemType::Income { .. }),
                                        )
                                        class=(
                                            "badge-expense",
                                            move || {
                                                matches!(item_type, LedgerItemType::Expenditure { .. })
                                            },
                                        )
                                    >
                                        {item_type_string}
                                    </td>
                                    <td>{amount}</td>
                                    <td>{description}</td>
                                    <td>{account}</td>
                                    <td>
                                        <button
                                            type="button"
                                            on:click=move |_| {
                                                match item_type {
                                                    LedgerItemType::Income { id } => {
                                                        budget
                                                            .update(move |b: &mut Budget| {
                                                                b.remove_income(&id);
                                                            })
                                                    }
                                                    LedgerItemType::Expenditure { id } => {
                                                        budget
                                                            .update(move |b: &mut Budget| {
                                                                b.remove_expenditure(&id);
                                                            })
                                                    }
                                                }
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
    }
}

trait IntoLedger {
    fn into_ledger(&self) -> Vec<LedgerItem>;
}

struct LedgerItem {
    item_type: LedgerItemType,
    item_type_string: String,
    amount: String,
    description: String,
    account: String,
}

#[derive(Clone, Copy)]
enum LedgerItemType {
    Income { id: IncomeId },
    Expenditure { id: ExpenditureId },
}

impl IntoLedger for Budget {
    fn into_ledger(&self) -> Vec<LedgerItem> {
        self.incomes()
            .iter()
            .map(|i| LedgerItem {
                item_type: LedgerItemType::Income { id: i.id },
                item_type_string: "Income".to_string(),
                amount: format_amount(i.amount),
                description: i.name.clone(),
                account: self.get_account(&i.account_id).unwrap().name.clone(),
            })
            .chain(self.expenditures().iter().map(|e| LedgerItem {
                item_type: LedgerItemType::Expenditure { id: e.id },
                item_type_string: "Outgoing".to_string(),
                amount: format_amount(e.amount),
                description: e.name.clone(),
                account: self.get_account(&e.account_id).unwrap().name.clone(),
            }))
            .collect()
    }
}

fn format_amount(amount: f64) -> String {
    format!("£{:.2}", amount)
}

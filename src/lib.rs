use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Budget {
    name: String,
    incomes: Vec<Income>,
    expenditures: Vec<Expenditure>,
    next_income_id: IncomeId,
    next_expenditure_id: ExpenditureId,
    next_account_id: AccountId,
    accounts: Vec<Account>,
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct IncomeId(u32);

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Income {
    pub id: IncomeId,
    pub name: String,
    pub amount: Decimal,
    pub account_id: AccountId,
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct ExpenditureId(u32);

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Expenditure {
    pub id: ExpenditureId,
    pub name: String,
    pub amount: Decimal,
    pub account_id: AccountId,
    pub commitment: Option<AccountCommitment>,
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct AccountId(pub u32);

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AccountCommitment {
    PullAuthorisation,
    ScheduledTransfer,
    Subscription,
}

impl Budget {
    pub fn new(name: String) -> Self {
        Budget {
            name,
            incomes: vec![],
            expenditures: vec![],
            next_income_id: IncomeId(0),
            next_expenditure_id: ExpenditureId(0),
            next_account_id: AccountId(0),
            accounts: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn incomes(&self) -> &[Income] {
        &self.incomes
    }

    pub fn expenditures(&self) -> &[Expenditure] {
        &self.expenditures
    }

    pub fn accounts(&self) -> &[Account] {
        &self.accounts
    }

    pub fn get_account(&self, account_id: &AccountId) -> Option<&Account> {
        self.accounts().iter().find(|a| a.id.eq(account_id))
    }

    pub fn total_income(&self) -> Decimal {
        self.incomes.iter().map(|i| i.amount).sum()
    }

    pub fn total_expenditure(&self) -> Decimal {
        self.expenditures.iter().map(|i| i.amount).sum()
    }

    pub fn balance(&self) -> Decimal {
        self.total_income() - self.total_expenditure()
    }

    pub fn add_income(
        &mut self,
        name: String,
        amount: Decimal,
        account_id: AccountId,
    ) -> Result<(), String> {
        if String::is_empty(&name) || name.chars().all(char::is_whitespace) {
            return Err("Name cannot be empty".to_string());
        }

        if self
            .incomes
            .iter()
            .any(|i| i.name.eq_ignore_ascii_case(&name))
        {
            return Err("Income with name already exists".to_string());
        }

        if self.accounts.iter().all(|i| i.id.ne(&account_id)) {
            return Err("Account not found in budget".to_string());
        }

        let income = Income::new(self.next_income_id, name, amount, account_id);

        self.next_income_id = IncomeId(self.next_income_id.0 + 1);

        self.incomes.push(income);

        Ok(())
    }

    pub fn add_expenditure(
        &mut self,
        name: String,
        amount: Decimal,
        account_id: AccountId,
        commitment: Option<AccountCommitment>,
    ) -> Result<(), String> {
        if String::is_empty(&name) || name.chars().all(char::is_whitespace) {
            return Err("Name cannot be empty".to_string());
        }

        if self
            .expenditures
            .iter()
            .any(|i| i.name.eq_ignore_ascii_case(&name))
        {
            return Err("Expenditure with name already exists".to_string());
        }

        if self.accounts.iter().all(|i| i.id.ne(&account_id)) {
            return Err("Account not found in budget".to_string());
        }

        let expenditure = Expenditure::new(
            self.next_expenditure_id,
            name,
            amount,
            account_id,
            commitment,
        );

        self.next_expenditure_id = ExpenditureId(self.next_expenditure_id.0 + 1);

        self.expenditures.push(expenditure);

        Ok(())
    }

    pub fn remove_income(&mut self, id: &IncomeId) {
        self.incomes.retain(|i| i.id != *id);
    }

    pub fn remove_expenditure(&mut self, id: &ExpenditureId) {
        self.expenditures.retain(|e| e.id != *id);
    }

    pub fn remove_account(&mut self, id: AccountId) -> Result<(), String> {
        if self.incomes.iter().any(|i| i.account_id.eq(&id)) {
            return Err("Incomes linked to account".to_string());
        }

        if self.expenditures.iter().any(|i| i.account_id.eq(&id)) {
            return Err("Expenditures linked to account".to_string());
        }

        self.accounts.retain(|i| i.id != id);

        Ok(())
    }

    pub fn add_account(&mut self, name: String) -> Result<(), String> {
        if String::is_empty(&name) || name.chars().all(char::is_whitespace) {
            return Err("Name cannot be empty".to_string());
        }

        if self
            .accounts
            .iter()
            .any(|i| i.name.eq_ignore_ascii_case(&name))
        {
            return Err("Account with name already exists".to_string());
        }

        let account = Account::new(self.next_account_id, name);

        self.next_account_id = AccountId(self.next_account_id.0 + 1);

        self.accounts.push(account);

        Ok(())
    }
}

impl Income {
    fn new(id: IncomeId, name: String, amount: Decimal, account_id: AccountId) -> Income {
        Income {
            id,
            name,
            amount,
            account_id,
        }
    }
}

impl Expenditure {
    fn new(
        id: ExpenditureId,
        name: String,
        amount: Decimal,
        account_id: AccountId,
        commitment: Option<AccountCommitment>,
    ) -> Expenditure {
        Expenditure {
            id,
            name,
            amount,
            account_id,
            commitment,
        }
    }
}

impl Account {
    fn new(id: AccountId, name: String) -> Account {
        Account { id, name }
    }
}

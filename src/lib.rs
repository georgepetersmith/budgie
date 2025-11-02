#[derive(Default, Clone)]
pub struct Budget {
    incomes: Vec<Income>,
    expenditures: Vec<Expenditure>,
}

#[derive(Default, Clone)]
pub struct Income {
    amount: f64,
}

#[derive(Default, Clone)]
pub struct Expenditure {
    amount: f64,
}

impl Budget {
    pub fn total_income(&self) -> f64 {
        self.incomes.iter().map(|i| i.amount).sum()
    }

    pub fn total_expenditure(&self) -> f64 {
        self.expenditures.iter().map(|i| i.amount).sum()
    }

    pub fn balance(&mut self) -> f64 {
        self.total_income() - self.total_expenditure()
    }

    pub fn add_income(&mut self, income: Income) {
        self.incomes.push(income);
    }

    pub fn add_expenditure(&mut self, expenditure: Expenditure) {
        self.expenditures.push(expenditure);
    }
}

impl Income {
    pub fn new(amount: f64) -> Income {
        Income { amount }
    }
}

impl Expenditure {
    pub fn new(amount: f64) -> Expenditure {
        Expenditure { amount }
    }
}

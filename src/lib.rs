#[derive(Default, Clone)]
pub struct Budget {
    incomes: Vec<Income>,
    expenditures: Vec<Expenditure>,
}

#[derive(Default, Clone)]
pub struct Income {
    pub name: String,
    pub amount: f64,
}

#[derive(Default, Clone)]
pub struct Expenditure {
    pub name: String,
    pub amount: f64,
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
        if self
            .incomes
            .iter()
            .any(|i| i.name.eq_ignore_ascii_case(&income.name))
        {
            return;
        }

        self.incomes.push(income);
    }

    pub fn add_expenditure(&mut self, expenditure: Expenditure) {
        if self
            .expenditures
            .iter()
            .any(|i| i.name.eq_ignore_ascii_case(&expenditure.name))
        {
            return;
        }

        self.expenditures.push(expenditure);
    }

    pub fn incomes(&self) -> &[Income] {
        &self.incomes
    }

    pub fn expenditures(&self) -> &Vec<Expenditure> {
        &self.expenditures
    }
}

impl Income {
    pub fn new(name: String, amount: f64) -> Income {
        Income { name, amount }
    }
}

impl Expenditure {
    pub fn new(name: String, amount: f64) -> Expenditure {
        Expenditure { name, amount }
    }
}

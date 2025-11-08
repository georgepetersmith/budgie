#[derive(Default, Clone)]
pub struct Budget {
    incomes: Vec<Income>,
    expenditures: Vec<Expenditure>,
    next_income_id: u32,
    next_expenditure_id: u32,
}

#[derive(Default, Clone)]
pub struct Income {
    pub id: u32,
    pub name: String,
    pub amount: f64,
}

#[derive(Default, Clone)]
pub struct Expenditure {
    pub id: u32,
    pub name: String,
    pub amount: f64,
}

impl Budget {
    pub fn incomes(&self) -> &[Income] {
        &self.incomes
    }

    pub fn expenditures(&self) -> &Vec<Expenditure> {
        &self.expenditures
    }

    pub fn total_income(&self) -> f64 {
        self.incomes.iter().map(|i| i.amount).sum()
    }

    pub fn total_expenditure(&self) -> f64 {
        self.expenditures.iter().map(|i| i.amount).sum()
    }

    pub fn balance(&mut self) -> f64 {
        self.total_income() - self.total_expenditure()
    }

    pub fn add_income(&mut self, name: String, amount: f64) -> Result<(), String> {
        if self
            .incomes
            .iter()
            .any(|i| i.name.eq_ignore_ascii_case(&name))
        {
            return Err("Income with name already exists".to_string());
        }

        let income = Income::new(self.next_income_id, name, amount);

        self.next_income_id = self.next_income_id + 1;

        self.incomes.push(income);

        Ok(())
    }

    pub fn add_expenditure(&mut self, name: String, amount: f64) -> Result<(), String> {
        if self
            .expenditures
            .iter()
            .any(|i| i.name.eq_ignore_ascii_case(&name))
        {
            return Err("Expenditure with name already exists".to_string());
        }

        let expenditure = Expenditure::new(self.next_expenditure_id, name, amount);

        self.next_expenditure_id = self.next_expenditure_id + 1;

        self.expenditures.push(expenditure);

        Ok(())
    }

    pub fn remove_income(&mut self, id: u32) {
        self.incomes.retain(|i| i.id != id);
    }

    pub fn remove_expenditure(&mut self, id: u32) {
        self.expenditures.retain(|e| e.id != id);
    }
}

impl Income {
    pub fn new(id: u32, name: String, amount: f64) -> Income {
        Income { id, name, amount }
    }
}

impl Expenditure {
    pub fn new(id: u32, name: String, amount: f64) -> Expenditure {
        Expenditure { id, name, amount }
    }
}

pub struct Account {
    pub name: String,
    pub sol_balance: f64,
    pub token_balance: f64,
}

impl Account {
    pub fn new(name: &str, sol_balance: f64) -> Self {
        Self {
            name: name.to_string(),
            sol_balance,
            token_balance: 0.0,
        }
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 1,
        holder_name: String::from("Gizem"),
        balance: 100.0,
    };

    let mut account2 = BankAccount {
        account_number: 2,
        holder_name: String::from("Gunes"),
        balance: 700.0,
    };

    account1.deposit(300.0);
    account2.withdraw(400.0);

    println!(
        "Account {} ({}) balance: ${}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );

    println!(
        "Account {} ({}) balance: ${}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds for withdrawal!");
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
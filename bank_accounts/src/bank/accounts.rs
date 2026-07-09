use indoc::formatdoc;

#[allow(dead_code)]
pub struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    pub fn new(owner: String, balance: i64) -> Self {
        Account {
            owner,
            balance
        }
    }
}

impl Account {
    pub fn check_balance(&self) {
        println!("CURRENT BALANCE: {}", self.balance);
    }

    pub fn withdraw(&mut self, amount: i64) -> i64 {
        println!(
            "{}",
            formatdoc!(
                "
                
                PROCESSING REQUEST
                AVAILABLE BALANCE: {}
                AMOUNT TO BE WITHDRAWN: {amount}
        ",
                self.balance
            )
        );

        if amount > 0 {
            if amount <= self.balance {
                self.balance -= amount;

                println!("TRANSACTION SUCCESSFUL");
            } else {
                println!(
                    "{}",
                    formatdoc!(
                        "
                    TRANSACTION FAILED
                    THE WITHDRAWAL AMOUNT IS HIGHER THAN EXPECTED
                    "
                    )
                );
            }
        } else {
            println!("THE WITHDRAWAL AMOUNT IS LOWER THAN EXPECTED");
        }

        self.balance
    }

    pub fn deposit(&mut self, amount: i64) -> i64 {
        println!(
            "{}",
            formatdoc!(
                "
                
            PROCESSING REQUEST
            AVAILABLE BALANCE: {}
            AMOUNT TO BE DEPOSITED: {amount}
            ",
                self.balance
            )
        );

        if amount > 0 {
            self.balance += amount;
        } else {
            println!("THE DEPOSIT AMOUNT IS LOWER THAN EXPECTED");
        }

        self.balance
    }
}

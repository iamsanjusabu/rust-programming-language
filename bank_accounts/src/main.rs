mod bank;

fn main() {

    let mut user1 = bank::accounts::Account::new(
        String::from("sanjusabu"),
        100
    );

    user1.check_balance();
    user1.withdraw(50);
    user1.check_balance();
    user1.deposit(100);
    user1.check_balance();
}

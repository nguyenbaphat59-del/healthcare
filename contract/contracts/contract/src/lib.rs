#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct HealthTokenContract;

#[contractimpl]
impl HealthTokenContract {

    // Mint token cho user (ví dụ: từ thiện cấp)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&Symbol::new(&env, "BALANCES"))
            .unwrap_or(Map::new(&env));

        let current = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, current + amount);

        env.storage().instance().set(&Symbol::new(&env, "BALANCES"), &balances);
    }

    // Transfer token giữa người dùng
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&Symbol::new(&env, "BALANCES"))
            .unwrap_or(Map::new(&env));

        let from_balance = balances.get(from.clone()).unwrap_or(0);
        if from_balance < amount {
            panic!("Not enough balance");
        }

        balances.set(from.clone(), from_balance - amount);

        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_balance + amount);

        env.storage().instance().set(&Symbol::new(&env, "BALANCES"), &balances);
    }

    // Thanh toán dịch vụ y tế
    pub fn pay_for_service(env: Env, user: Address, hospital: Address, amount: i128) {
        user.require_auth();

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&Symbol::new(&env, "BALANCES"))
            .unwrap_or(Map::new(&env));

        let user_balance = balances.get(user.clone()).unwrap_or(0);
        if user_balance < amount {
            panic!("Insufficient funds");
        }

        // trừ token người dùng
        balances.set(user.clone(), user_balance - amount);

        // cộng token cho bệnh viện
        let hospital_balance = balances.get(hospital.clone()).unwrap_or(0);
        balances.set(hospital, hospital_balance + amount);

        env.storage().instance().set(&Symbol::new(&env, "BALANCES"), &balances);
    }

    // Xem số dư
    pub fn balance(env: Env, user: Address) -> i128 {
        let balances: Map<Address, i128> =
            env.storage().instance().get(&Symbol::new(&env, "BALANCES"))
            .unwrap_or(Map::new(&env));

        balances.get(user).unwrap_or(0)
    }
}
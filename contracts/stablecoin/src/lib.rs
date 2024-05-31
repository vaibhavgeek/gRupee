// The standard library is disabled to optimize the contract for low-resource environments like blockchain.
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, BytesN, Env, IntoVal, Vec};

// The DataKey enum is used to represent state variables stored in the contract's storage.
// This allows for structured access to data within the contract's storage.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Contributions(Address),
    Contributors,
    Token,
    GLRToken,
    IsActive,
    Admin,
    Initialized,
    ExchangePrice
}

// The `#[contract]` attribute marks a type as being the type that contract functions are attached for.
#[contract]
pub struct GlobalRupee;

// The `#[contractimpl]` exports the publicly accessible functions to the Soroban environment.
#[contractimpl]
impl GlobalRupee {
    /// Initialize the contract with the admin address and the deposit token contract address.
    /// Deploys the share token contract and initializes it.
    ///
    /// # Arguments
    /// - `env` - The execution environment of the contract.
    /// - `admin` - The address of the admin.
    /// - `token_wasm_hash` - The hash of the token contract wasm.
    /// - `token` - The address of the deposit token contract.
    pub fn initialize(env: Env, admin: Address, token_wasm_hash: BytesN<32>, token: Address) {
        // Sets the admin address in the storage.
        env.storage().instance().set(&DataKey::Admin, &admin);
        // Deploys the share token contract.
        let glr_contract = token::create_contract(&env, token_wasm_hash, &token);
        // Initializes the share token contract.
        token::Client::new(&env, &glr_contract).initialize(
            &env.current_contract_address(),
            &18u32,
            &"Global Rupee".into_val(&env),
            &"GLR".into_val(&env),
        );
        // Sets the token and share token addresses in the storage.
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage()
            .instance()
            .set(&DataKey::GLRToken, &glr_contract);
        // Sets the initialized status to initialized.
        env.storage().instance().set(&DataKey::Initialized, &true);
    }

   
    /// Records a deposit made by a contributor if the staking is active.
    /// # Arguments
    ///
    /// - `env` - The execution environment of the contract.
    /// - `contributor` - The address of the contributor making the contribution.
    /// - `token` - The address of the token to deposit.
    /// - `amount` - The amount of contribution in tokens.
    pub fn deposit(env: Env, contributor: Address, token: Address, amount: i128, exchange_price: i128) {
        contributor.require_auth();
        let transfer_amount = amount * 120 / 100;
        token::Client::new(&env, &token).transfer(
            &contributor,
            &env.current_contract_address(),
            &transfer_amount,
        );
        // Mint the share token to the contributor
        let share_token = Self::get_share_token(env.clone());
        Self::set_exchange_price(env.clone(), exchange_price);
        let share_amount = amount * exchange_price;
        token::Client::new(&env, &share_token).mint(&contributor, &share_amount);
        // Update the contribution in the storage
        Self::set_contribution(env.clone(), contributor.clone(), amount);

    }
    /// Withdraws the contribution made by a contributor if the staking is active.
    ///
    /// # Arguments
    /// - `env` - The execution environment of the contract.
    /// - `contributor` - The address of the contributor making the contribution.
    /// - `amount` - The amount of contribution in tokens.
    /// - `token` - The address of the token to withdraw.
    /// - `recipient` - The address of the recipient of the contribution.
    pub fn withdraw(env: Env, contributor: Address, recipient: Address, token: Address) {
        contributor.require_auth();
        // import Status enum from staking module
        // if !Self::is_contributor(env.clone(), contributor.clone()) {
        //     panic!("contributor has not contributed");
        // }
        let contribution = Self::get_user_contribution(env.clone(), contributor.clone());
        let exchange_price: i128 = Self::get_exchange_price(env.clone());
        // Transfer the contribution to the recipient
        let withdraw_amount = contribution * exchange_price;
        token::Client::new(&env, &token).transfer(
            &env.current_contract_address(),
            &recipient,
            &contribution,
        );
        // // Burn the share token
        let share_token = Self::get_share_token(env.clone());
        token::Client::new(&env, &share_token).burn(&contributor, &withdraw_amount);
    }

    pub fn set_exchange_price(env: Env, exchange_price: i128){
        env.storage()
        .instance()
        .set(&DataKey::ExchangePrice, &exchange_price);
    }
    pub fn get_exchange_price(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::ExchangePrice)
            .unwrap_or(0)
    }

    /// Clear the contributor from the storage
    pub fn clear_contributor(env: Env, contributor: Address) {
        env.storage()
            .instance()
            .remove(&DataKey::Contributions(contributor));
    }

    // Get a users total contribution
    pub fn get_user_contribution(env: Env, contributor: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Contributions(contributor))
            .unwrap_or(0)
    }

    // Set a users contribution
    pub fn set_contribution(env: Env, contributor: Address, amount: i128) {
        env.storage()
            .instance()
            .set(&DataKey::Contributions(contributor), &amount);
    }

    // Get the list of contributors
    pub fn get_contributors(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Contributors)
            .unwrap_or(vec![&env, env.current_contract_address()])
    }

    // Get the total contributions
    pub fn get_total_contributions(env: Env) -> i128 {
        let contributors = Self::get_contributors(env.clone());
        let mut total = 0;
        for contributor in contributors.iter() {
            total += Self::get_user_contribution(env.clone(), contributor.clone());
        }
        total
    }

    // Get the ShareToken address
    pub fn get_share_token(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::GLRToken)
            .unwrap_or(env.current_contract_address())
    }

    // Get user's share balance
    pub fn get_share_token_balance(env: Env, user: Address) -> i128 {
        let share_token = Self::get_share_token(env.clone());
        token::Client::new(&env, &share_token).balance(&user)
    }

   
    // Check if a user is a contributor
    pub fn is_contributor(env: Env, contributor: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Contributions(contributor))
            .unwrap_or(false)
    }
    // Add a new admin
    pub fn add_new_admin(env: Env, new_admin: Address) {
        Self::update_admin(env, new_admin);
    }

    // Sets the new admin address in the storage.
    fn update_admin(env: Env, new_admin: Address) {
        let current_admin = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or(env.current_contract_address());
        current_admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    // Get the admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or(env.current_contract_address())
    }
}

// imports Token module
mod token;

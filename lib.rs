#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod hellocontract {
    use ink_storage::traits::SpreadAllocate;
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Hellocontract {
        hello_map: ink_storage::Mapping<AccountId, u32>,
    }

    impl Hellocontract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(Self::new_init)
        }

        fn new_init(&mut self) {
            let caller = Self::env().caller();
            let value: u32 = Default::default();
            self.hello_map.insert(&caller, &value);
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        #[ink(message)]
        pub fn add(&mut self, count: u32) {
            let caller = self.env().caller();
            let current_value = self.hello_map.get(&caller).unwrap_or_default();
            let new_value = count + current_value;
            self.hello_map.insert(&caller, &new_value);
        }

        #[ink(message)]
        pub fn get_value(&self, caller: AccountId) -> u32 {
            self.hello_map.get(&caller).unwrap_or_default()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::test;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        fn set_caller(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }

        fn default_accounts() -> test::DefaultAccounts<Environment> {
            ink_env::test::default_accounts::<Environment>()
        }

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let hellocontract = Hellocontract::default();
            let accounts = default_accounts();
            assert_eq!(hellocontract.get_value(accounts.alice), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let accounts = default_accounts();
            let mut hellocontract = Hellocontract::new();
            assert_eq!(hellocontract.get_value(accounts.alice), 0);
            hellocontract.add(20);
            assert_eq!(hellocontract.get_value(accounts.alice), 20);
        }
    }
}

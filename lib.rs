#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod time_lock {
    use ink_core::storage;
    use ink_core::env;


    // Storage of the contract
    #[ink(storage)]
    struct TimeLock {
        release_time: storage::Value<u64>,
        saving_account: storage::HashMap<AccountId, Balance>,
    }

    impl TimeLock {
        #[ink(constructor)]
        fn savings_account(&mut self) {
            self.release_time.set(0 as u64);
        }

        // Deposit function, enter the release time in ms
        #[ink(message)]
        fn deposit(&mut self, reltime_ms: u64) -> bool {
            let callee = self.env().caller(); // Get current contract caller
            let value = env::transferred_balance::<env::DefaultEnvTypes>().unwrap_or(0); // Get the value of the deposit try with self.env().transferred_balance().unwrap_or(0) in the future.
            
            // For now only 1 account for contract deployment
            if !self.saving_account.is_empty() {
                return false
            }

            // Sets a saving account
            self.saving_account.insert(callee, value); 

            // Get block time to calculate the release time
            let block_time = self.get_blocktime();
            self.release_time.set(block_time + reltime_ms); // Sets release time
            true
        }

        // Withdrawal function, only possible if blocktime is more than release time
        #[ink(message)]
        fn withdrawal(&mut self, amount: Balance) -> Result< bool, &'static str> {
            if self.release_time > self.get_blocktime() {
                return Err("Release time not met.")
            }

            let callee = self.env().caller(); // Get current contract caller
            if self.saving_account.get(&callee) == None {
                return Err("Only original caller can withdraw.")
            }

            // Current savings value
            let value  = self.get_account_balance();

            // Transfer the funds
            if amount > value {
                return Err("Cant withdraw more that you have.")
            } else if amount < value {
                // Transfer an amount but keep savings account
                self.env().transfer(callee, value - amount).unwrap_or(());
                self.saving_account.insert(callee, value-amount);
            } else {
                // If equal, transfer all and remove savings account
                self.env().transfer(callee, value).unwrap_or(());
                self.saving_account.remove(&callee);
                self.release_time.set(0); // Sets release time
            }
            Ok(true)
        }
        
        // Get Balance of Savings Account
        #[ink(message)]
        fn get_account_balance(&self) -> Balance {
            let callee = self.env().caller();
            *self.saving_account.get(&callee).unwrap_or(&0)
        }

        // Get the current balance of contract
        #[ink(message)]
        fn get_contract_balance(&self) -> Balance {
            let balance = env::balance::<env::DefaultEnvTypes>().unwrap_or(0); //Try with self.env().balance() in the future.
            return balance;
        }

        // Get current block time in ms
        #[ink(message)]
        fn get_blocktime(&self) -> u64 {
            return env::block_timestamp::<env::DefaultEnvTypes>().unwrap_or(0); //Try with self.env().block_timestamp() in the future.
        }
        
        // Get the release time of the savings account
        #[ink(message)]
        fn get_release_time(&self) -> Result<u64, &'static str> {
            let block_time = self.get_blocktime();

            let callee = self.env().caller(); // Get current contract caller
            if self.saving_account.get(&callee) == None {
                return Err("You have no account.")
            }

            Ok(*self.release_time - block_time)
        }
        
        // Check if account has desposit
        #[ink(message)]
        fn has_deposit(&self) -> bool {
            let callee = self.env().caller();
            if self.saving_account.get(&callee) == None {
                return false
            };
            true
        }

        /* TEST FOR GAS
        #[ink(message)]
        fn gas_price(&self) -> Balance {
            self.env().gas_price()
        }   

        #[ink(message)]
        fn gas_left(&self) -> Balance {
            self.env().gas_left()
        }*/
    }
    /*
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn no_deposit_deploy() {
            let accounts = env::test::default_accounts::<env::DefaultEnvTypes>()
            .expect("Cannot get accounts");
            let TimeLock = TimeLock::new();
            assert_eq!(TimeLock.has_deposit(), false);
        }
    }*/
}

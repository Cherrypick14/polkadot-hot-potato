#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34)]
#[ink::contract]
mod hotpotato {
    use openbrush::{
        contracts::psp34::*,
        traits::Storage,
    };
    use ink::{
        prelude::vec::Vec,
        primitives::AccountId,
    };

    /// Hot Potato Game Contract Errors
    #[derive(Debug, PartialEq, Eq, ink::scale::Encode, ink::scale::Decode)]
    #[cfg_attr(feature = "std", derive(ink::scale_info::TypeInfo))]
    pub enum HotPotatoError {
        /// Game is not active
        GameNotActive,
        /// Game is already active
        GameAlreadyActive,
        /// Deadline has passed
        DeadlinePassed,
        /// Only current holder can pass the potato
        NotCurrentHolder,
        /// PSP34 Error
        PSP34Error(PSP34Error),
    }

    impl From<PSP34Error> for HotPotatoError {
        fn from(error: PSP34Error) -> Self {
            HotPotatoError::PSP34Error(error)
        }
    }

    /// Hot Potato Game Result Type
    pub type Result<T> = core::result::Result<T, HotPotatoError>;

    /// The Hot Potato Game Contract Storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Hotpotato {
        #[storage_field]
        psp34: psp34::Data,
        /// Current holder of the potato NFT
        current_holder: Option<AccountId>,
        /// Block number when potato was last passed
        last_passed_block: u32,
        /// Number of blocks before deadline
        deadline_blocks: u32,
        /// Whether the game is currently active
        active: bool,
        /// Token ID counter for minting
        next_token_id: u128,
    }

    impl Hotpotato {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new(deadline_blocks: u32) -> Self {
            let mut instance = Self::default();
            instance.deadline_blocks = deadline_blocks;
            instance.next_token_id = 1;
            instance
        }

        /// Start a new hot potato game by minting an NFT to the specified account
        #[ink(message)]
        pub fn start_game(&mut self, to: AccountId) -> Result<()> {
            if self.active {
                return Err(HotPotatoError::GameAlreadyActive);
            }

            // Mint the potato NFT
            let token_id = Id::U128(self.next_token_id);
            self._mint_to(to, token_id)?;
            self.next_token_id += 1;

            // Set game state
            self.current_holder = Some(to);
            self.last_passed_block = self.env().block_number();
            self.active = true;

            Ok(())
        }

        /// Pass the potato to another account
        #[ink(message)]
        pub fn pass_potato(&mut self, to: AccountId) -> Result<()> {
            if !self.active {
                return Err(HotPotatoError::GameNotActive);
            }

            let caller = self.env().caller();

            // Check if caller is current holder
            if self.current_holder != Some(caller) {
                return Err(HotPotatoError::NotCurrentHolder);
            }

            // Check if deadline has passed
            let current_block = self.env().block_number();
            if current_block > self.last_passed_block + self.deadline_blocks {
                return Err(HotPotatoError::DeadlinePassed);
            }

            // Transfer the potato NFT
            let token_id = Id::U128(1); // We only have one potato token
            self._transfer(caller, to, token_id, Vec::new())?;

            // Update game state
            self.current_holder = Some(to);
            self.last_passed_block = current_block;

            Ok(())
        }

        /// Check deadline and burn potato if time expired
        #[ink(message)]
        pub fn check_deadline(&mut self) -> Result<bool> {
            if !self.active {
                return Ok(false);
            }

            let current_block = self.env().block_number();
            if current_block > self.last_passed_block + self.deadline_blocks {
                // Burn the potato NFT
                let token_id = Id::U128(1);
                self._burn_from(self.current_holder.unwrap(), token_id)?;

                // Reset game state
                self.current_holder = None;
                self.active = false;

                return Ok(true);
            }

            Ok(false)
        }

        /// Get the current holder of the potato
        #[ink(message)]
        pub fn get_holder(&self) -> Option<AccountId> {
            self.current_holder
        }

        /// Get game status
        #[ink(message)]
        pub fn is_active(&self) -> bool {
            self.active
        }

        /// Get deadline blocks
        #[ink(message)]
        pub fn get_deadline_blocks(&self) -> u32 {
            self.deadline_blocks
        }

        /// Get last passed block
        #[ink(message)]
        pub fn get_last_passed_block(&self) -> u32 {
            self.last_passed_block
        }
    }

    /// Unit tests for the Hot Potato Game Contract
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let hotpotato = Hotpotato::new(10);
            assert_eq!(hotpotato.get_deadline_blocks(), 10);
            assert!(!hotpotato.is_active());
            assert_eq!(hotpotato.get_holder(), None);
        }

        #[ink::test]
        fn start_game_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut hotpotato = Hotpotato::new(10);

            assert!(hotpotato.start_game(accounts.alice).is_ok());
            assert!(hotpotato.is_active());
            assert_eq!(hotpotato.get_holder(), Some(accounts.alice));
        }

        #[ink::test]
        fn start_game_twice_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut hotpotato = Hotpotato::new(10);

            assert!(hotpotato.start_game(accounts.alice).is_ok());
            assert_eq!(hotpotato.start_game(accounts.bob), Err(HotPotatoError::GameAlreadyActive));
        }

        #[ink::test]
        fn pass_potato_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut hotpotato = Hotpotato::new(10);

            // Set caller to alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            assert!(hotpotato.start_game(accounts.alice).is_ok());
            assert!(hotpotato.pass_potato(accounts.bob).is_ok());
            assert_eq!(hotpotato.get_holder(), Some(accounts.bob));
        }

        #[ink::test]
        fn pass_potato_not_holder_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut hotpotato = Hotpotato::new(10);

            // Start game with alice as holder
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(hotpotato.start_game(accounts.alice).is_ok());

            // Try to pass from bob (not holder)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(hotpotato.pass_potato(accounts.charlie), Err(HotPotatoError::NotCurrentHolder));
        }
    }


    /// End-to-end tests for the Hot Potato Game Contract
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn new_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = HotpotatoRef::new(10);

            // When
            let contract = client
                .instantiate("hotpotato", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Hotpotato>();

            // Then
            let is_active = call_builder.is_active();
            let is_active_result = client.call(&ink_e2e::alice(), &is_active).dry_run().await?;
            assert!(!is_active_result.return_value());

            let get_holder = call_builder.get_holder();
            let get_holder_result = client.call(&ink_e2e::alice(), &get_holder).dry_run().await?;
            assert_eq!(get_holder_result.return_value(), None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn start_game_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = HotpotatoRef::new(10);
            let contract = client
                .instantiate("hotpotato", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Hotpotato>();

            // When
            let start_game = call_builder.start_game(ink_e2e::alice().account_id());
            let _start_result = client
                .call(&ink_e2e::alice(), &start_game)
                .submit()
                .await
                .expect("start_game failed");

            // Then
            let is_active = call_builder.is_active();
            let is_active_result = client.call(&ink_e2e::alice(), &is_active).dry_run().await?;
            assert!(is_active_result.return_value());

            let get_holder = call_builder.get_holder();
            let get_holder_result = client.call(&ink_e2e::alice(), &get_holder).dry_run().await?;
            assert_eq!(get_holder_result.return_value(), Some(ink_e2e::alice().account_id()));

            Ok(())
        }
    }
}

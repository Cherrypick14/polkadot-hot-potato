#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod hotpotato {

    /// Hot Potato Game Contract Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum HotPotatoError {
        /// Game is not active
        GameNotActive,
        /// Game is already active
        GameAlreadyActive,
        /// Deadline has passed
        DeadlinePassed,
        /// Only current holder can pass the potato
        NotCurrentHolder,
    }

    /// Hot Potato Game Result Type
    pub type Result<T> = core::result::Result<T, HotPotatoError>;

    /// 🎯 EVENTS - Will add back after basic contract works
    /// For now, frontend can query contract state directly

    /// 📦 Contract Storage
    #[ink(storage)]
    pub struct Hotpotato {
        /// Current holder of the potato
        current_holder: Option<AccountId>,
        /// Block when potato was last passed
        last_passed_block: u32,
        /// Blocks until deadline
        deadline_blocks: u32,
        /// Is game active?
        active: bool,
        /// Who started the game
        game_starter: Option<AccountId>,
    }

    impl Hotpotato {
        /// Constructor - Initialize the hot potato game
        #[ink(constructor)]
        pub fn new(deadline_blocks: u32) -> Self {
            Self {
                current_holder: None,
                last_passed_block: 0,
                deadline_blocks,
                active: false,
                game_starter: None,
            }
        }

        /// Helper function to convert caller to AccountId
        fn caller_to_account_id(&self) -> AccountId {
            // For simplicity, we'll create a deterministic AccountId from the caller
            // In a real contract, you might want a more sophisticated conversion
            let caller = self.env().caller();
            let mut account_id = [0u8; 32];
            // Copy the 20 bytes from caller to the first 20 bytes of AccountId
            account_id[..20].copy_from_slice(caller.as_ref());
            AccountId::from(account_id)
        }

        /// 🚀 Start a new hot potato game
        /// This demonstrates event emission in ink!
        #[ink(message)]
        pub fn start_game(&mut self, to: AccountId) {
            assert!(!self.active, "Game already active");

            let caller = self.caller_to_account_id();
            let current_block = self.env().block_number();

            // Update contract state
            self.current_holder = Some(to);
            self.last_passed_block = current_block;
            self.active = true;
            self.game_starter = Some(caller);

            // 🎯 EMIT EVENT - Commented out for now
            // self.env().emit_event(GameAction {
            //     action_type: 1, // start
            //     player: caller,
            // });
        }

        /// 🔄 Pass the potato to another account
        /// Shows conditional event emission
        #[ink(message)]
        pub fn pass_potato(&mut self, to: AccountId) {
            assert!(self.active, "Game not active");

            let caller = self.caller_to_account_id();
            assert_eq!(self.current_holder, Some(caller), "Not current holder");

            let current_block = self.env().block_number();

            // Check deadline
            assert!(
                current_block <= self.last_passed_block + self.deadline_blocks,
                "Deadline passed"
            );

            // Calculate remaining blocks
            let _remaining_blocks = (self.last_passed_block + self.deadline_blocks)
                .saturating_sub(current_block);

            // Update state
            self.current_holder = Some(to);
            self.last_passed_block = current_block;

            // 🎯 EMIT EVENT - Commented out for now
            // self.env().emit_event(GameAction {
            //     action_type: 2, // pass
            //     player: caller,
            // });
        }

        /// 💥 Check deadline and burn potato if expired
        /// Demonstrates multiple event emissions in one function
        #[ink(message)]
        pub fn check_deadline(&mut self) -> bool {
            if !self.active {
                return false;
            }

            let current_block = self.env().block_number();

            if current_block > self.last_passed_block + self.deadline_blocks {
                let _last_holder = self.current_holder.unwrap();

                // 🎯 EMIT EVENT - Commented out for now
                // self.env().emit_event(GameAction {
                //     action_type: 3, // end
                //     player: last_holder,
                // });

                // Reset game
                self.reset_game();
                return true;
            }

            false
        }

        /// 🏆 End game manually (only by starter)
        #[ink(message)]
        pub fn end_game(&mut self) {
            assert!(self.active, "Game not active");

            let caller = self.caller_to_account_id();
            assert_eq!(self.game_starter, Some(caller), "Only starter can end");

            let _current_block = self.env().block_number();
            let _has_winner = self.current_holder.is_some();

            // 🎯 EMIT EVENT - Commented out for now
            // self.env().emit_event(GameAction {
            //     action_type: 3, // end
            //     player: caller,
            // });

            self.reset_game();
        }

        /// Helper function to reset game state
        fn reset_game(&mut self) {
            self.current_holder = None;
            self.active = false;
            self.game_starter = None;
        }

        /// 📖 Query functions (no events needed for reads)

        #[ink(message)]
        pub fn get_holder(&self) -> Option<AccountId> {
            self.current_holder
        }

        #[ink(message)]
        pub fn is_active(&self) -> bool {
            self.active
        }

        #[ink(message)]
        pub fn get_deadline_blocks(&self) -> u32 {
            self.deadline_blocks
        }

        #[ink(message)]
        pub fn get_last_passed_block(&self) -> u32 {
            self.last_passed_block
        }

        #[ink(message)]
        pub fn get_game_starter(&self) -> Option<AccountId> {
            self.game_starter
        }

        #[ink(message)]
        pub fn get_remaining_blocks(&self) -> u32 {
            if !self.active {
                return 0;
            }
            let current_block = self.env().block_number();
            (self.last_passed_block + self.deadline_blocks).saturating_sub(current_block)
        }
    }

    /// 🧪 Unit Tests - Testing Events and Game Logic
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
            let accounts = ink::env::test::default_accounts();
            let mut hotpotato = Hotpotato::new(10);

            // Create simple test AccountIds
            let alice = AccountId::from([0x01; 32]);
            let bob = AccountId::from([0x02; 32]);

            // Set caller to alice (using H160 for test environment)
            ink::env::test::set_caller(accounts.alice);

            // Start game - should not panic
            hotpotato.start_game(bob);

            // Verify state
            assert!(hotpotato.is_active());
            assert_eq!(hotpotato.get_holder(), Some(bob));
            assert_eq!(hotpotato.get_game_starter(), Some(alice));
        }

        #[ink::test]
        #[should_panic(expected = "Game already active")]
        fn start_game_twice_panics() {
            let accounts = ink::env::test::default_accounts();
            let mut hotpotato = Hotpotato::new(10);

            let bob = AccountId::from([0x02; 32]);
            let charlie = AccountId::from([0x03; 32]);

            ink::env::test::set_caller(accounts.alice);

            hotpotato.start_game(bob);
            hotpotato.start_game(charlie); // Should panic
        }

        #[ink::test]
        fn pass_potato_works() {
            let accounts = ink::env::test::default_accounts();
            let mut hotpotato = Hotpotato::new(10);

            let bob = AccountId::from([0x02; 32]);
            let charlie = AccountId::from([0x03; 32]);

            // Alice starts game with Bob as holder
            ink::env::test::set_caller(accounts.alice);
            hotpotato.start_game(bob);

            // Bob passes to Charlie
            ink::env::test::set_caller(accounts.bob);
            hotpotato.pass_potato(charlie);

            assert_eq!(hotpotato.get_holder(), Some(charlie));
        }

        #[ink::test]
        #[should_panic(expected = "Not current holder")]
        fn pass_potato_wrong_holder_panics() {
            let accounts = ink::env::test::default_accounts();
            let mut hotpotato = Hotpotato::new(10);

            let alice = AccountId::from([0x01; 32]);
            let bob = AccountId::from([0x02; 32]);

            // Alice starts, Bob is holder
            ink::env::test::set_caller(accounts.alice);
            hotpotato.start_game(bob);

            // Charlie tries to pass (but he's not holder)
            ink::env::test::set_caller(accounts.charlie);
            hotpotato.pass_potato(alice); // Should panic
        }

        #[ink::test]
        fn end_game_works() {
            let accounts = ink::env::test::default_accounts();
            let mut hotpotato = Hotpotato::new(10);

            let bob = AccountId::from([0x02; 32]);

            // Alice starts and ends game
            ink::env::test::set_caller(accounts.alice);
            hotpotato.start_game(bob);
            hotpotato.end_game();

            assert!(!hotpotato.is_active());
            assert_eq!(hotpotato.get_holder(), None);
        }

        #[ink::test]
        fn check_deadline_no_active_game() {
            let mut hotpotato = Hotpotato::new(10);
            assert!(!hotpotato.check_deadline());
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
        async fn start_game_emits_event(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = HotpotatoRef::new(10);
            let contract = client
                .instantiate("hotpotato", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Hotpotato>();

            // When
            let start_game = call_builder.start_game(ink_e2e::bob().account_id());
            let start_result = client
                .call(&ink_e2e::alice(), &start_game)
                .submit()
                .await
                .expect("start_game failed");

            // Then - Check that GameStarted event was emitted
            let events = start_result.events;
            assert!(!events.is_empty(), "Expected GameStarted event to be emitted");

            // Check game state
            let is_active = call_builder.is_active();
            let is_active_result = client.call(&ink_e2e::alice(), &is_active).dry_run().await?;
            assert!(is_active_result.return_value());

            let get_holder = call_builder.get_holder();
            let get_holder_result = client.call(&ink_e2e::alice(), &get_holder).dry_run().await?;
            assert_eq!(get_holder_result.return_value(), Some(ink_e2e::bob().account_id()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn pass_potato_emits_event(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = HotpotatoRef::new(10);
            let contract = client
                .instantiate("hotpotato", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Hotpotato>();

            // Start game with Bob as initial holder
            let start_game = call_builder.start_game(ink_e2e::bob().account_id());
            let _start_result = client
                .call(&ink_e2e::alice(), &start_game)
                .submit()
                .await
                .expect("start_game failed");

            // When - Bob passes potato to Charlie
            let pass_potato = call_builder.pass_potato(ink_e2e::charlie().account_id());
            let pass_result = client
                .call(&ink_e2e::bob(), &pass_potato)
                .submit()
                .await
                .expect("pass_potato failed");

            // Then - Check that PotatoPassed event was emitted
            let events = pass_result.events;
            assert!(!events.is_empty(), "Expected PotatoPassed event to be emitted");

            // Check game state
            let get_holder = call_builder.get_holder();
            let get_holder_result = client.call(&ink_e2e::alice(), &get_holder).dry_run().await?;
            assert_eq!(get_holder_result.return_value(), Some(ink_e2e::charlie().account_id()));

            Ok(())
        }
    }
}

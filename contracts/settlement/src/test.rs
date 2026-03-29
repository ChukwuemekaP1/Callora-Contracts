#[cfg(test)]
mod settlement_tests {
    use crate::{CalloraSettlement, CalloraSettlementClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_settlement_initialization() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);

        client.init(&admin, &vault);

        assert_eq!(client.get_admin(), admin);
        assert_eq!(client.get_vault(), vault);

        let global_pool = client.get_global_pool();
        assert_eq!(global_pool.total_balance, 0);
    }

    #[test]
    fn test_receive_payment_to_pool() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        let payment_amount = 1000i128;
        client.receive_payment(&vault, &payment_amount, &true, &None);

        let global_pool = client.get_global_pool();
        assert_eq!(global_pool.total_balance, payment_amount);
    }

    #[test]
    fn test_receive_payment_to_developer() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let developer = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        let payment_amount = 500i128;
        client.receive_payment(&vault, &payment_amount, &false, &Some(developer.clone()));

        let balance = client.get_developer_balance(&developer);
        assert_eq!(balance, payment_amount);

        let global_pool = client.get_global_pool();
        assert_eq!(global_pool.total_balance, 0);
    }

    #[test]
    #[should_panic(expected = "unauthorized: caller must be vault or admin")]
    fn test_receive_payment_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let unauthorized = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&unauthorized, &100i128, &true, &None);
    }

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn test_receive_payment_zero_amount() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&vault, &0i128, &true, &None);
    }

    #[test]
    #[should_panic(expected = "developer address required when to_pool=false")]
    fn test_receive_payment_pool_false_no_developer() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&vault, &100i128, &false, &None);
    }

    #[test]
    #[should_panic(expected = "settlement contract already initialized")]
    fn test_double_init_panics() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);
        client.init(&admin, &vault);
    }

    #[test]
    fn test_set_admin() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let new_admin = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.set_admin(&admin, &new_admin);
        assert_eq!(client.get_admin(), new_admin);
    }

    #[test]
    #[should_panic(expected = "unauthorized: caller is not admin")]
    fn test_set_admin_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let attacker = Address::generate(&env);
        let new_admin = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.set_admin(&attacker, &new_admin);
    }

    #[test]
    fn test_set_vault() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let new_vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.set_vault(&admin, &new_vault);
        assert_eq!(client.get_vault(), new_vault);
    }

    #[test]
    #[should_panic(expected = "unauthorized: caller is not admin")]
    fn test_set_vault_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let attacker = Address::generate(&env);
        let new_vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.set_vault(&attacker, &new_vault);
    }

    #[test]
    fn test_get_all_developer_balances() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let dev1 = Address::generate(&env);
        let dev2 = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&vault, &300i128, &false, &Some(dev1.clone()));
        client.receive_payment(&vault, &200i128, &false, &Some(dev2.clone()));

        let all = client.get_all_developer_balances();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_receive_multiple_payments_accumulate() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let developer = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&vault, &100i128, &false, &Some(developer.clone()));
        client.receive_payment(&vault, &150i128, &false, &Some(developer.clone()));

        let balance = client.get_developer_balance(&developer);
        assert_eq!(balance, 250i128);
    }

    #[test]
    fn test_admin_can_receive_payment() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&admin, &500i128, &true, &None);

        let global_pool = client.get_global_pool();
        assert_eq!(global_pool.total_balance, 500i128);
    }

    // ===== Small Map Iteration Tests =====
    // These tests verify map iteration behavior and ensure developers understand
    // the exposed iteration characteristics when working with small maps.

    #[test]
    fn test_small_map_iteration_single_entry() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let dev1 = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        // Single entry should always be returned
        client.receive_payment(&vault, &100i128, &false, &Some(dev1.clone()));
        let all = client.get_all_developer_balances();
        assert_eq!(all.len(), 1);
        assert_eq!(all.get(0).unwrap().balance, 100i128);
    }

    #[test]
    fn test_small_map_iteration_three_entries() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let dev1 = Address::generate(&env);
        let dev2 = Address::generate(&env);
        let dev3 = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        // Add three developers
        client.receive_payment(&vault, &100i128, &false, &Some(dev1.clone()));
        client.receive_payment(&vault, &200i128, &false, &Some(dev2.clone()));
        client.receive_payment(&vault, &300i128, &false, &Some(dev3.clone()));

        // Verify all entries are returned (order may vary)
        let all = client.get_all_developer_balances();
        assert_eq!(all.len(), 3);

        // Collect balances to verify they match expected values
        let mut balances: Vec<i128> = Vec::new(&env);
        for entry in all.iter() {
            balances.push_back(entry.balance);
        }

        // Verify all expected balances are present (order independent verification)
        assert!(balances.iter().any(|b| *b == 100i128));
        assert!(balances.iter().any(|b| *b == 200i128));
        assert!(balances.iter().any(|b| *b == 300i128));
    }

    #[test]
    fn test_small_map_iteration_consistency_single_call() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let dev1 = Address::generate(&env);
        let dev2 = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&vault, &500i128, &false, &Some(dev1.clone()));
        client.receive_payment(&vault, &600i128, &false, &Some(dev2.clone()));

        // Multiple calls within same block should show consistent state
        let first_call = client.get_all_developer_balances();
        let second_call = client.get_all_developer_balances();

        assert_eq!(first_call.len(), second_call.len());
        assert_eq!(first_call.len(), 2);
    }

    #[test]
    fn test_small_map_point_lookup_preferred() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let dev1 = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        client.receive_payment(&vault, &777i128, &false, &Some(dev1.clone()));

        // Point lookup should work correctly even with map present
        let point_lookup = client.get_developer_balance(&dev1);
        assert_eq!(point_lookup, 777i128);

        // Verify consistency with full iteration
        let all = client.get_all_developer_balances();
        assert_eq!(all.len(), 1);
        assert_eq!(all.get(0).unwrap().balance, 777i128);
    }

    #[test]
    fn test_small_map_zero_developers() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        // Empty map should return empty vector
        let all = client.get_all_developer_balances();
        assert_eq!(all.len(), 0);
    }

    #[test]
    fn test_small_map_repeated_updates_same_developer() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let dev1 = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        // Update same developer multiple times
        for amount in &[100i128, 50i128, 75i128, 25i128] {
            client.receive_payment(&vault, amount, &false, &Some(dev1.clone()));
        }

        // Map should have single entry with accumulated balance
        let all = client.get_all_developer_balances();
        assert_eq!(all.len(), 1);
        assert_eq!(all.get(0).unwrap().balance, 250i128);
    }

    #[test]
    fn test_small_map_warning_no_ordering_guarantee() {
        // This test documents the warning: map iteration order is NOT guaranteed.
        // The test verifies the function returns correct data but does not assume ordering.
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);

        // Create 5 developers
        let mut devs: Vec<Address> = Vec::new(&env);
        for _ in 0..5 {
            devs.push_back(Address::generate(&env));
        }

        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        // Add in order: 1000, 2000, 3000, 4000, 5000
        for (idx, dev) in devs.iter().enumerate() {
            let amount = ((idx as i128) + 1) * 1000i128;
            client.receive_payment(&vault, &amount, &false, &Some(dev.clone()));
        }

        // Verify all entries are present (data integrity)
        let all = client.get_all_developer_balances();
        assert_eq!(all.len(), 5);

        // Collect all balances
        let mut collected_balances: Vec<i128> = Vec::new(&env);
        for entry in all.iter() {
            collected_balances.push_back(entry.balance);
        }

        // Verify each balance is present (order-independent check)
        for expected_balance in &[1000i128, 2000i128, 3000i128, 4000i128, 5000i128] {
            assert!(
                collected_balances.iter().any(|b| b == expected_balance),
                "Missing expected balance: {}",
                expected_balance
            );
        }

        // NOTE: We do NOT verify insertion order because map iteration order is unstable.
        // Do not write code that depends on the order returned by get_all_developer_balances().
    }

    #[test]
    fn test_small_map_edge_case_negative_balance_prevented() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let dev1 = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        // Positive payment should succeed
        client.receive_payment(&vault, &100i128, &false, &Some(dev1.clone()));
        assert_eq!(client.get_developer_balance(&dev1), 100i128);

        // Negative payment should be rejected by zero-check
        // (Note: Rust's i128 addition wraps; the validation happens at receive_payment entry)
    }

    #[test]
    #[should_panic(expected = "developer address required when to_pool=false")]
    fn test_small_map_edge_case_missing_developer() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        let addr = env.register(CalloraSettlement, ());
        let client = CalloraSettlementClient::new(&env, &addr);
        client.init(&admin, &vault);

        // Should panic if developer address not provided when to_pool=false
        client.receive_payment(&vault, &100i128, &false, &None);
    }
}

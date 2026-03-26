use soroban_sdk::{symbol_short, vec, Address, BytesN, Env, Symbol};

pub fn pay_premium(env: Env, _policy_id: BytesN<32>) {
    // Note: get_killswitch_id should be provided by the environment or configuration
    // For now, we assume a storage-backed address or generate a test one.
    let killswitch_id: Address = env.storage().instance().get(&symbol_short!("KS_ID")).unwrap_or_else(|| Address::generate(&env));
    let is_paused: bool = env.invoke_contract(&killswitch_id, &symbol_short!("is_paused"), vec![&env, Symbol::new(&env, "insurance")].into());
    
    if is_paused {
        panic!("Contract is currently paused for emergency maintenance.");
    }
    // ... rest of the logic
}
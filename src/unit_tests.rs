/*
    Create a contract under user1. 
    Then, Check if the state contains the contract associated with user1
*/
#[test]
fn test_create_contract() {
   let mut deps = mock_dependencies(&[]);
   let env = mock_env("creator", &[]);
   let info = mock_info("creator", &[]);

   // Create a contract
   let msg = ExecuteMsg::Create {
       underlying: "ATOM".to_string(),
       strike_price: 100_000000,
       expiration: env.block.time.plus_seconds(100).nanos(), // Expires in 100 seconds
       contract_type: "call".to_string(),
       bid_price: 50_000000,
       ask_price: 150_000000,
   };

   let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
   assert_eq!(res.messages.len(), 0);

   // Query the state to ensure the contract is in the contracts map
   let state = State::from_storage(&mut deps.storage).unwrap();
   let contract_id = info.sender.to_string();
   assert!(state.contracts.contains_key(&contract_id));
   let contract = state.contracts.get(&contract_id).unwrap();
   assert_eq!(contract.owner, info.sender.to_string());
   assert_eq!(contract.underlying, "ATOM");
   assert_eq!(contract.strike_price, 100_000000);
   assert_eq!(contract.contract_type, "call");
   assert_eq!(contract.bid_price, 50_000000);
   assert_eq!(contract.ask_price, 150_000000);
}



/*
    Create a contract under user1 and have user2 bid on it. 
    Check the state to see if the contract's bid amount is updated under user1. 
*/ 

#[test]
fn test_bid_on_contract() {
   let mut deps = mock_dependencies(&[]);
   let env = mock_env("creator", &[]);
   let creator_info = mock_info("creator", &[]);
   let bidder_info = mock_info("bidder", &[]);

   // Create a contract
   let msg = ExecuteMsg::Create {
       underlying: "ATOM".to_string(),
       strike_price: 100_000000,
       expiration: env.block.time.plus_seconds(100).nanos(), 
       contract_type: "call".to_string(),
       bid_price: 50_000000,
       ask_price: 150_000000,
   };

   let _res = execute(deps.as_mut(), env.clone(), creator_info.clone(), msg).unwrap();

   // Bid on the contract
   let contract_id = creator_info.sender.to_string();
   let msg = ExecuteMsg::Bid {
       contract_id: contract_id.clone(),
       bid_amount: 200_000000,
   };

   let _res = execute(deps.as_mut(), env.clone(), bidder_info.clone(), msg).unwrap();

   // Query the state to ensure the contract's bid amount is updated
   let state = State::from_storage(&mut deps.storage).unwrap();
   let contract = state.contracts.get(&contract_id).unwrap();
   assert_eq!(contract.bid_price, 200_000000);
   assert_eq!(contract.owner, bidder_info.sender.to_string());
}




/* 
    Create a contract under user1, 
    bid on it from user2, 
    then have user1 execute the contract to transfer ownership to user2.
*/

#[test]
fn test_execute_contract() {
   let mut deps = mock_dependencies(&[]);
   let env = mock_env("creator", &[]);
   let creator_info = mock_info("creator", &[]);
   let bidder_info = mock_info("bidder", &[]);

   // Create a contract
   let msg = ExecuteMsg::Create {
       underlying: "ATOM".to_string(),
       strike_price: 100_000000,
       expiration: env.block.time.plus_seconds(100).nanos(),  
       contract_type: "call".to_string(),
       bid_price: 50_000000,
       ask_price: 150_000000,
   };

   let _res = execute(deps.as_mut(), env.clone(), creator_info.clone(), msg).unwrap();

   // Bid on the contract
   let contract_id = creator_info.sender.to_string();
   let msg = ExecuteMsg::Bid {
       contract_id: contract_id.clone(),
       bid_amount: 200_000000,
   };

   let _res = execute(deps.as_mut(), env.clone(), bidder_info.clone(), msg).unwrap();

   // Execute the contract
   let msg = ExecuteMsg::Execute {
       contract_id: contract_id.clone(),
   };

   let _res = execute(deps.as_mut(), env.clone(), creator_info.clone(), msg).unwrap();

   // Query the state to ensure the contract's ownership has been transferred
   let state = State::from_storage(&mut deps.storage).unwrap();
   let contract = state.contracts.get(&contract_id).unwrap();
   assert_eq!(contract.owner, bidder_info.sender.to_string());
}



/*
    User1 creates a contract and then expires it - deleting it from the state.
*/

#[test]
fn test_expire_contract() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env("creator", &[]);
    let info = mock_info("creator", &[]);

    // Create a contract
    let msg = ExecuteMsg::Create {
        underlying: "ATOM".to_string(),
        strike_price: 100_000000,
        expiration: env.block.time.plus_seconds(100).nanos(),  
        contract_type: "call".to_string(),
        bid_price: 50_000000,
        ask_price: 150_000000,
    };

    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Expire the contract
    let contract_id = info.sender.to_string();
    let msg = ExecuteMsg::Expire {
        contract_id: contract_id.clone(),
    };

    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query the state to ensure the contract is no longer in the contracts map
    let state = State::from_storage(&mut deps.storage).unwrap();
    assert!(!state.contracts.contains_key(&contract_id));
}
// use cosmwasm_schema::serde::{Deserialize, Serialize};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage,
};
use std::collections::HashMap;

// Define the struct representing an options contract
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
//
#[cw_serde]
pub struct OptionsContract {
    pub owner: String,
    pub underlying: String,
    pub strike_price: u64,
    pub expiration: u64,
    pub contract_type: String, // e.g., "call" or "put"
    pub bid_price: u64,
    pub ask_price: u64,
}

// Define the contract state
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct State {
    pub contracts: HashMap<String, OptionsContract>,
}

impl State {
    pub fn from_storage(storage: &mut dyn Storage) -> StdResult<Self> {
        let data = storage.get(b"state");
        match data {
            Some(bytes) => cosmwasm_std::from_binary(&Binary::from(bytes)),
            None => Ok(State {
                contracts: HashMap::new(),
            }),
        }
    }

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        storage.set(b"state", &cosmwasm_std::to_binary(self)?);
        Ok(())
    }
}

// Entry point for creating a new options contract
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    underlying: String,
    strike_price: u64,
    expiration: u64,
    contract_type: String,
    bid_price: u64,
    ask_price: u64,
) -> StdResult<Response> {
    let contract = OptionsContract {
        owner: info.sender.to_string(),
        underlying,
        strike_price,
        expiration,
        contract_type,
        bid_price,
        ask_price,
    };

    let mut state = State {
        contracts: HashMap::new(),
    };
    let contract_id = contract.owner.clone();
    state.contracts.insert(contract_id, contract);

    deps.storage.set(b"state", &to_binary(&state)?);

    Ok(Response::default())
}

// Entry point for transferring ownership of an options contract
pub fn transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract_id: String,
    new_owner: String,
) -> StdResult<Response> {
    let mut state = State::from_storage(deps.storage)?;
    let contract = state
        .contracts
        .get(&contract_id)
        .ok_or_else(|| StdError::not_found("Contract not found"))?;

    if contract.owner != info.sender {
        return Err(StdError::generic_err("Not authorized to transfer contract"));
    }

    // contract.owner = new_owner;
    let mut new_contract = contract.clone();
    new_contract.owner = new_owner;
    state.contracts.insert(contract_id, new_contract);
    deps.storage.set(b"state", &to_binary(&state)?);

    Ok(Response::default())
}

// Entry point for expiring an options contract
pub fn expire(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract_id: String,
) -> StdResult<Response> {
    let mut state = State::from_storage(deps.storage)?;
    let contract = state
        .contracts
        .get(&contract_id)
        .ok_or_else(|| StdError::not_found("Contract not found"))?;

    if contract.owner != info.sender {
        return Err(StdError::generic_err("Not authorized to expire contract"));
    }

    state.contracts.remove(&contract_id);
    deps.storage.set(b"state", &to_binary(&state)?);

    Ok(Response::default())
}

// Entry point for placing a bid on an options contract
pub fn bid(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract_id: String,
    bid_amount: u64,
) -> StdResult<Response> {
    let mut state = State::from_storage(deps.storage)?;
    let contract = state
        .contracts
        .get(&contract_id)
        .ok_or_else(|| StdError::not_found("Contract not found"))?;

    if bid_amount < contract.ask_price {
        return Err(StdError::generic_err(
            "Bid amount is lower than the current ask price",
        ));
    }

    // Update the bid price and owner
    let mut new_contract = contract.clone();
    new_contract.bid_price = bid_amount;
    new_contract.owner = info.sender.to_string();

    state.contracts.insert(contract_id, new_contract);
    deps.storage.set(b"state", &to_binary(&state)?);

    Ok(Response::default())
}

// Entry point for executing an options contract
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract_id: String,
) -> StdResult<Response> {
    let mut state = State::from_storage(deps.storage)?;
    let mut contract = state
        .contracts
        .get_mut(&contract_id)
        .ok_or_else(|| StdError::not_found("Contract not found"))?;

    if contract.owner != info.sender {
        return Err(StdError::generic_err("Not authorized to execute contract"));
    }

    if contract.bid_price >= contract.ask_price {
        // Transfer ownership and funds
        // let new_owner = contract.owner.clone();
        // contract.owner = info.sender.to_string();
        let mut new_contract = contract.clone();
        new_contract.owner = info.sender.to_string();

        // Transfer funds from the new owner to the old owner
        // (Implement your fund transfer logic here)

        state.contracts.insert(contract_id, new_contract);
        deps.storage.set(b"state", &to_binary(&state)?);
    } else {
        return Err(StdError::generic_err(
            "No bids match or exceed the ask price",
        ));
    }

    Ok(Response::default())
}

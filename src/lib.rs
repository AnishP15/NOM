// use crate::msgs::ExecuteMsg;
// use crate::msgs::messages;
// use crate::msgs;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Storage, WasmMsg,
};
use std::collections::HashMap;

pub mod msgs;
use msgs::ExecuteMsg;

// Define the struct representing an options contract
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo) -> StdResult<Response> {
    let mut state = State {
        contracts: HashMap::new(),
    };

    deps.storage.set(b"state", &to_binary(&state)?);

    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
    underlying: String,
    strike_price: u64,
    expiration: u64,
    contract_type: String,
    bid_price: u64,
    ask_price: u64,
) -> StdResult<Response> {
    let mut state = State::from_storage(deps.storage)?;
    match msg {
        ExecuteMsg::Create {
            underlying,
            strike_price,
            expiration,
            contract_type,
            bid_price,
            ask_price,
        } => {
            let contract = OptionsContract {
                owner: info.sender.to_string(),
                underlying,
                strike_price,
                expiration,
                contract_type,
                bid_price,
                ask_price,
            };
            let contract_id = contract.owner.clone();
            state.contracts.insert(contract_id, contract);

            Ok(Response::default())
        }

        // Entry point for expiring an options contract
        ExecuteMsg::Expire { contract_id } => {
            // let mut state = State::from_storage(deps.storage)?;
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
        ExecuteMsg::Bid {
            contract_id,
            bid_amount,
        } => {
            // let mut state = State::from_storage(&mut deps.storage)?;
            let mut contract = state
                .contracts
                .get_mut(&contract_id)
                .ok_or_else(|| StdError::not_found("Contract not found"))?;

            if bid_amount < contract.ask_price {
                return Err(StdError::generic_err(
                    "Bid amount is lower than the current ask price",
                ));
            }

            let mut new_contract = contract.clone();
            // Update the bid price and owner
            new_contract.bid_price = bid_amount;
            new_contract.owner = info.sender.to_string();

            state.contracts.insert(contract_id, new_contract);
            deps.storage.set(b"state", &to_binary(&state)?);

            Ok(Response::default())
        }

        // Entry point for executing an options contract
        ExecuteMsg::Execute { contract_id } => {
            // let mut state = State::from_storage(deps.storage)?;
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
    }
}

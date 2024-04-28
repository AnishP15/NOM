pub enum ExecuteMsg {
    Expire {
        contract_id: String,
    },
    Bid {
        contract_id: String,
        bid_amount: u64,
    },
    Execute {
        contract_id: String,
    },
    Create {
        underlying: String,
        strike_price: u64,
        expiration: u64,
        contract_type: String,
        bid_price: u64,
        ask_price: u64,
    },
}

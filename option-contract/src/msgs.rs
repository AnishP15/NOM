pub enum ExecuteMsg {
    Transfer { 
        contract_id: String,
        new_owner: String 
    },

    Expire { contract_id: String },
    Bid { 
        contract_id: String,
        bid_amount: u64
    },
    Execute { contract_id: String, },
}

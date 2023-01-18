library errors;

pub enum AccessError {
    AlreadyInitialized: (),
    NoNftDeposited: (),
    NotNftAdmin: (),
}

pub enum AssetError {
    SupplyNotReturned: (),
}

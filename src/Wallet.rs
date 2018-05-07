struct Wallet {
    private_key: String,
    public_key: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        (private_key, public_key) = Self::generate_keys();
        
        Wallet {
            private_key,
            public_key,
        }
    }
}

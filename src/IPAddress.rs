pub trait IpAddress {
    fn to_string(&self) -> String;
}

pub struct IPV4Address {
    address: String,
}

impl IPV4Address {
    pub fn new(address: String) -> IPV4Address {
        IPV4Address {
            address: address,
        }
    }
}

impl IpAddress for IPV4Address {
    fn to_string(&self) -> String {
        self.address.to_string()
    }
}

// Placeholder
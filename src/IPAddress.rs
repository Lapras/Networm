pub mod ipaddress {
    struct IPV4 {
        address: String,
    }

    impl IPV4 {
        fn toString(&self) -> String {
            address;
        }
    }


    struct IPV6 {
        address: String,
    }

    impl IPV6 {
        fn toString(&self) -> String {
            address;
        }
    }

    pub enum IPAddress {
        V4 (IPV4),
        V6 (IPV6),
    }
}
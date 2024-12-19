use std::fmt;

pub struct Machine {
    address: String,
    os: OperatingSystem,
    machine_type: MachineType,
}

pub enum OperatingSystem {
    Linux,
    Windows,
    Mac,
}

pub enum MachineType {
    Physical,
    Virtual, 
    Containerized,
}

impl Machine {
    pub fn new(address: String) -> Machine {
        Machine {
            address: address,
            os: OperatingSystem::Linux,
            machine_type: MachineType::Physical,
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f :&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address)
    }
}
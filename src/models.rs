use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Domains {
    domain: String,
    base_64_aes: String,
}

#[derive(Serialize, Deserialize)]
pub struct Relay {
    multicast_address: String,
    multicast_port: u16,
    network_interface: String,
    configured_domains: [Domains; 2]
}

impl Domains {
    pub fn new(domain:String, base_64_aes:String) -> Domains {
        Domains {
            domain,
            base_64_aes,
        }
    }

    pub fn get_domain(self) -> String{
        self.domain
    }

    pub fn get_base(self) -> String {
        self.base_64_aes
    }
}

impl Relay {
    pub fn new(multicast_address:String, multicast_port:u16, network_interface:String, configured_domains:[Domains; 2]) -> Relay {
        Relay {
            multicast_address,
            multicast_port,
            network_interface,
            configured_domains
        }
    }

    pub fn get_multicast_address(self) -> String {
        self.multicast_address
    }

    pub fn get_multicast_port(self) -> u16 {
        self.multicast_port
    }

    pub fn get_network_interface(self) -> String {
        self.network_interface
    }

    pub fn get_configured_domains(self) -> [Domains; 2] {
        self.configured_domains
    }

    pub fn to_string(self) -> String {
        return format!("{}, {}, {}, {}", self.multicast_address, self.multicast_port, self.network_interface, self.network_interface);
    }
}

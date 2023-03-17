
pub(crate) mod dto {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct DomainsDto {
        domain: String,
        base_64_aes: String,
    }

    impl DomainsDto{

        pub fn get_domain(&self) -> String {
            self.domain.clone()
        }

        pub fn get_aeskey(&self) -> String {
            self.base_64_aes.clone()
        }
    }

    impl Clone for DomainsDto {
        fn clone(&self) -> Self {
            DomainsDto {
                domain: self.domain.clone(),
                base_64_aes: self.base_64_aes.clone()
            }
        }
    }

    #[derive(Deserialize)]
    pub struct RelayDto {
        multicast_address: String,
        multicast_port: u16,
        network_interface: String,
        configured_domains: Vec<DomainsDto>
    }

    impl RelayDto {

        pub fn get_multicast_address(&self) -> String {
            self.multicast_address.clone()
        }

        pub fn get_multicast_port(&self) -> u16 {
            self.multicast_port.clone()
        }

        pub fn get_network_interface(&self) -> String {
            self.network_interface.clone()
        }

        pub fn get_configured_domains(&self) -> Vec<DomainsDto> {
            self.configured_domains.clone()
        }
    }

    impl Clone for RelayDto {
        fn clone(&self) -> Self {
            RelayDto {
                multicast_address: self.multicast_address.clone(),
                multicast_port: self.multicast_port.clone(),
                network_interface: self.network_interface.clone(),
                configured_domains: self.configured_domains.clone(),
            }
        }
    }
}

mod mapper {
    use crate::infrastructure::dto::{DomainsDto, RelayDto};
    use crate::models::{Domains, Relay};

    fn dto_to_domains(domain_dto: DomainsDto) -> Domains {
        let cloned_to_domains = domain_dto.clone();

        Domains::new(cloned_to_domains.get_domain().clone(), cloned_to_domains.get_aeskey().clone())
    }

    pub fn dto_to_relay(relay_dto: RelayDto) -> Relay {
        let cloned_relay_dto = relay_dto.clone();

        let mut configured_domain: Vec<Domains> = Vec::new();

        for d in cloned_relay_dto.get_configured_domains() {
            configured_domain.push(dto_to_domains(d))
        }

        Relay::new(cloned_relay_dto.get_multicast_address().clone(),
                   cloned_relay_dto.get_multicast_port().clone(),
                   cloned_relay_dto.get_network_interface().clone(),
                   configured_domain.clone())
    }

}

pub(super) mod json_repository {
    use crate::infrastructure::mapper::dto_to_relay;
    use crate::models::Relay;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use crate::infrastructure::dto::RelayDto;

    pub fn get_relay() -> Relay {
        let file_path = Path::new("./data/relay.json");
        let display_path = file_path.display();

        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(e) => panic!("Erreur lors de l'ouverture du fichier config {}: {}", display_path, e),
        };

        let mut json = String::new();
        match file.read_to_string(&mut json) {
            Ok(_) => println!("Json chargé {} \n", display_path),
            Err(e) => panic!("Erreur lors de la lecture du fichier config {}: {}", display_path, e),
        };

        println!("{}",&*json.clone());
        let relay_dto: RelayDto = serde_json::from_str(&json).unwrap();

        dto_to_relay(relay_dto)
    }

}
pub(super) mod dto {

    pub struct DomainsDto {
        domain: String,
        base: String,
    }

    pub struct RelayDto {
        multicast_address: String,
        multicast_port: u16,
        network_interface: String,
        configured_domains: [DomainsDto; 2]
    }

    impl DomainsDto{
        pub fn new(domain:String, base:String) -> Self {
            Self {
                domain,
                base
            }
        }
    }

    impl RelayDto {
        pub fn new(multicast_address: String, multicast_port: u16, network_interface: String, configured_domains: [DomainsDto; 2]) -> Self {
            Self {
                multicast_address,
                multicast_port,
                network_interface,
                configured_domains
            }
        }
    }
}

mod json {
    use crate::infrastructure::dto::{DomainsDto, RelayDto};
    use crate::models::{Domains, Relay};

    fn domain_to_dto(domain:Domains) -> DomainsDto {
        DomainsDto::new(String::from(domain.get_domain()), String::from(domain.get_base()))
    }

    fn relay_to_dto(relay:Relay) -> RelayDto {
        let domains = relay.get_configured_domains();
        let dto_domains : [DomainsDto; 2] = [domain_to_dto(domains[0]), domain_to_dto(domains[1])];
        RelayDto::new(String::from(relay.get_multicast_address()), relay.get_multicast_port(),
                      String::from(relay.get_network_interface()), dto_domains)
    }

    /* TODO
    fn dto_to_domains() -> Domains {

    }

    fn dto_to_relay() -> Relay {

    }
     */

}
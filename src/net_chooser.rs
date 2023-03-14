use network_interface::{NetworkInterface};
use network_interface::NetworkInterfaceConfig;
use std::io::{stdin};
use std::{usize};

pub struct NetWorkInterface {
    network_interface: String,
    multicast_address: String
}

pub struct NetWorkInterfacesList {
    list: Vec<NetWorkInterface>
}

impl NetWorkInterface {
    pub fn new(network_interface: String, multicast_address: String) -> NetWorkInterface {
        NetWorkInterface {
            network_interface,
            multicast_address
        }
    }

    pub fn get_network_interface(&self) -> String {
        self.network_interface.clone()
    }

    pub fn get_multicast_address(&self) -> String {
        self.multicast_address.clone()
    }

    /*pub fn to_string(&self) -> String {
        return format!("{}, {}", self.network_interface, self.multicast_address);
    }*/
}

impl NetWorkInterfacesList{
    pub fn new() -> NetWorkInterfacesList {
        NetWorkInterfacesList {
            list: NetWorkInterfacesList::get_network_interfaces()
        }
    }

    /*pub fn add(&mut self, network_interface: String, multicast_address: String) {
        self.List.push(NetWorkInterface::new(network_interface, multicast_address));
    }*/

    fn get_network_interfaces() -> Vec<NetWorkInterface> {
        let mut result = Vec::new();

        let network_interfaces = NetworkInterface::show().unwrap();

        for itf in network_interfaces.iter() {
            println!("{:?}", itf.name);
            for it in itf.addr.iter() {
                if it.ip().is_ipv4() {
                    result.push(NetWorkInterface::new(itf.name.clone(), it.ip().to_string()));
                }
            }
        }
        result
    }

    pub fn choose(&mut self) -> &NetWorkInterface {
        for i in 0..self.list.len() {
            println!("{} : {}", i, self.list[i].network_interface.clone());
        }
        println!("Select your interface : ");
        let selected = self.list.get(NetWorkInterfacesList::read_input() as usize).unwrap().clone();
        println!("Selected interface : {}", selected.get_network_interface());
        return selected;
    }

    pub fn read_input() -> u32 {
        let mut input_text = String::new();
        stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        let trimmed = input_text.trim();
        let index : u32;
        match trimmed.parse::<u32>() {
            Ok(i) => index= i,
            Err(..) => index = trimmed.parse().unwrap(),
        };
        return index;
    }
}


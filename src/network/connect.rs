use stun::{Client, IpVersion, Message, Attribute, XorMappedAddress};
use std::net::{SocketAddr, IpAddr};
use pnet::datalink;

#[derive(Debug)]
pub enum ConnectionError {
    StunFailed
}

pub fn init(server: &str, local_port: u16) -> Result<SocketAddr, ConnectionError> {
    let client = Client::new(server, local_port, IpVersion::V4);
    let mesage = Message::request();
    let encoded = mesage.encode();
    let response = client.send(encoded.clone());
    let response = Message::decode(response);
    for attribute in response.attributes {
        match attribute {
            Attribute::XorMappedAddress(XorMappedAddress(a)) => {
                return Ok(a);
            }

            _ => {

            }
        }
    }
    return Err(ConnectionError::StunFailed);
}

pub fn find_interface() {
    for iface in datalink::interfaces() {
        println!("{:?}", iface);
    }
}
use core::fmt;

use std::io::{self, Read};
use std::net::UdpSocket;

use macaddr::MacAddr6;

#[derive(Debug)]
pub struct MagicPacket {
    packet: Vec<u8>,
}

impl MagicPacket {
    pub fn from_mac(mac: MacAddr6) -> Self {
        let preamble = &[0xFF; 6];
        let magic_seq = mac.into_array().repeat(16);

        let packet = [preamble, magic_seq.as_slice()].concat();

        MagicPacket { packet }
    }

    pub fn send(&self) -> Result<(), io::Error> {
        let socket = UdpSocket::bind("0.0.0.0")?;
        socket.set_broadcast(true)?;
        socket.send_to(self.packet.as_slice(), "255.255.255.255:9")?;

        Ok(())
    }
}

impl fmt::Display for MagicPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let address_bytes = &self.packet[6..12];
        let mut address = address_bytes
            .bytes()
            .map(|x| x.unwrap())
            .map(|x| format!("{:02X?}", x))
            .fold(String::from(""), |acc, e| acc + "-" + &e);

        address.remove(0);
        write!(f, "{}", address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_format_check() {
        let address = MacAddr6::new(0x01, 0x23, 0x45, 0x67, 0x89, 0xAB);
        let packet = MagicPacket::from_mac(address);

        assert_eq!(packet.to_string(), "01-23-45-67-89-AB");
    }

    #[test]
    fn address_format_check_nil() {
        let address = MacAddr6::nil();
        let packet = MagicPacket::from_mac(address);

        assert_eq!(packet.to_string(), "00-00-00-00-00-00");
    }

    #[test]
    fn address_format_check_broadcast() {
        let address = MacAddr6::broadcast();
        let packet = MagicPacket::from_mac(address);

        assert_eq!(packet.to_string(), "FF-FF-FF-FF-FF-FF");
    }
}

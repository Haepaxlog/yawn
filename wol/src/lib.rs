use core::fmt;
use std::io::{self, Read};
use std::net::UdpSocket;

use macaddr::MacAddr6;

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
        let address = self.packet[6..11]
            .bytes()
            .map(|x| x.unwrap())
            .map(|x| x.to_string())
            .fold(String::from(""), |acc: String, e: String| acc + "-" + &e);

        write!(f, "{}", address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}

use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Id {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub name: String,
}

impl Id {
    pub fn new(ip: Ipv4Addr, name: String, port: u16) -> Id {
        Self {
            ip: ip,
            name: name,
            port: port,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let name = self.name.as_bytes();
        let mut buf = Vec::with_capacity(2 + name.len() + 4);
        // name len + name
        buf.push(name.len() as u8);
        buf.extend_from_slice(name);
        buf.extend_from_slice(&self.ip.octets());

        buf.extend_from_slice(&self.port.to_be_bytes());
        buf
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() {
            return Err("Empty packet");
        }

        let mut pos = 0;

        let name_len = bytes[pos] as usize;
        pos += 1;

        if bytes.len() < pos + name_len + 4 + 2 {
            return Err("Packet too short");
        }

        let name = std::str::from_utf8(&bytes[pos..pos + name_len])
            .map_err(|_| "Invalid UTF-8")?
            .to_owned();
        pos += name_len;

        let ip = Ipv4Addr::new(bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]);
        pos += 4;

        let port = u16::from_be_bytes([bytes[pos], bytes[pos + 1]]);

        Ok(Id { ip, name, port })
    }
}

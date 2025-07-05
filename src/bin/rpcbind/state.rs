use rpcbind_rs::xdr_types::rpcbind::RPCB;

use crate::netconfig::NET_CONFIG;
use std::{collections::HashMap, net::SocketAddrV4};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramKey {
    pub program: u32,
    pub version: u32,
    pub net_id: String,
}

impl ProgramKey {
    pub fn portmapper_description(&self) -> Option<u32> {
        pub const IPPROTO_TCP: u32 = 6;
        pub const IPPROTO_UDP: u32 = 17;

        for net_config in NET_CONFIG.iter() {
            if net_config.network_id == self.net_id {
                return Some(match net_config.protoname.as_str() {
                    "tcp" => IPPROTO_TCP,
                    "udp" => IPPROTO_UDP,
                    _ => {
                        continue;
                    }
                });
            }
        }
        None
    }
}

impl From<&RPCB> for ProgramKey {
    fn from(value: &RPCB) -> Self {
        Self {
            program: value.r_prog,
            version: value.r_vers,
            net_id: value.r_netid.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ProgramDescription {
    pub addr: SocketAddrV4,
    pub owner: Option<String>,
}

impl ProgramDescription {
    pub fn universal_address(&self) -> String {
        let port = self.addr.port().to_be_bytes();
        format!("{}.{}.{}", self.addr.ip(), port[0], port[1])
    }
}

pub type State = HashMap<ProgramKey, ProgramDescription>;

pub fn make_rpcb((key, value): (&ProgramKey, &ProgramDescription)) -> RPCB {
    RPCB {
        r_prog: key.program,
        r_vers: key.version,
        r_netid: key.net_id.clone(),
        r_addr: value.universal_address(),
        r_owner: value.owner.clone().unwrap_or_else(String::new),
    }
}

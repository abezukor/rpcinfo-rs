use std::{
    collections::hash_map::Entry,
    net::{Ipv4Addr, SocketAddrV4},
};

use facet::Facet;
use onc_rpc::AcceptedStatus;
use rpcbind_rs::{RpcBindResult, request::RpcRequest};

use crate::{
    RPCResult, STATE,
    error::AcceptedStatusError,
    state::{ProgramDescription, ProgramKey},
};

mod portmapper;
mod rpcbind;

type RequestResult = RPCResult<Vec<u8>>;

pub fn process_request(request: &RpcRequest) -> RequestResult {
    match request {
        RpcRequest::V2(port_mapper_request) => portmapper::process_request(port_mapper_request),
        RpcRequest::V3(rpc_bind_request) | RpcRequest::V4(rpc_bind_request) => {
            rpcbind::process_request(rpc_bind_request)
        }
    }
}

fn set(key: ProgramKey, val: ProgramDescription) -> RequestResult {
    let mut state = STATE.write();
    let entry = state.entry(key);
    let result = match entry {
        Entry::Occupied(_) => false,
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(val);
            true
        }
    };
    serialize_result(&result)
}

fn decode_universal_address(universal_address: &str) -> RpcBindResult<SocketAddrV4> {
    // See https://datatracker.ietf.org/doc/html/rfc5665#autoid-13
    #[inline]
    fn take_byte<'a, 'b>(iter: &'a mut impl Iterator<Item = &'b str>) -> RpcBindResult<u8> {
        let byte_str = iter.next().ok_or(AcceptedStatus::GarbageArgs)?;
        byte_str
            .parse::<u8>()
            .map_err(|_| AcceptedStatus::GarbageArgs)
    }

    let mut split = universal_address.split('.');

    let mut ip_addr = split.by_ref().take(4);
    let (a, b, c, d) = (
        take_byte(&mut ip_addr)?,
        take_byte(&mut ip_addr)?,
        take_byte(&mut ip_addr)?,
        take_byte(&mut ip_addr)?,
    );
    let ip_addr = Ipv4Addr::new(a, b, c, d);

    let port = [take_byte(&mut split)?, take_byte(&mut split)?];

    Ok(SocketAddrV4::new(ip_addr, u16::from_be_bytes(port)))
}

#[inline]
fn serialize_result<'f, Res: Facet<'f>>(res: &'f Res) -> RequestResult {
    Ok(facet_xdr::to_vec(res).map_err(|_| AcceptedStatusError::SystemError)?)
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use super::decode_universal_address;
    use crate::state::ProgramDescription;

    #[test]
    fn address_decoder_test() {
        let test_addr = SocketAddrV4::new(Ipv4Addr::new(0x01, 0x23, 0x45, 0x67), 0xB3A2);
        let description = ProgramDescription {
            addr: test_addr,
            owner: None,
        };

        let universal_address = description.universal_address();
        assert_eq!(universal_address, "1.35.69.103.179.162");

        let decoded_addr = decode_universal_address(&universal_address).unwrap();

        assert_eq!(test_addr, decoded_addr);
    }
}

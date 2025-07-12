use std::net::{Ipv4Addr, SocketAddrV4};

use rpcbind_rs::{
    request::PortMapperRequest,
    xdr_types::{
        CreateList,
        port_mapper::{Mapping, PMapList},
    },
};

use super::{RequestResult, serialize_result};
use crate::{
    STATE,
    error::AcceptedStatusError,
    state::{ProgramDescription, ProgramKey},
};

pub fn process_request(request: &PortMapperRequest) -> RequestResult {
    #[allow(unused_variables)]
    match request {
        PortMapperRequest::Null => Ok(Vec::new()),
        PortMapperRequest::Set(mapping) => set(mapping),
        PortMapperRequest::Unset(mapping) => unset(mapping),
        PortMapperRequest::GetPort(mapping) => get_port(mapping),
        PortMapperRequest::Dump => dump(),
        PortMapperRequest::CallIt(call_args) => todo!(),
    }
}

fn set(mapping: &Mapping) -> RequestResult {
    let key = ProgramKey::from(mapping);
    let port = mapping
        .port
        .try_into()
        .map_err(|_| AcceptedStatusError::GarbageArgs)?;
    let val = ProgramDescription {
        addr: SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port),
        owner: None,
    };

    super::set(key, val)
}

fn unset(mapping: &Mapping) -> RequestResult {
    let mut state = STATE.write();
    let key = ProgramKey::from(mapping);
    // Protocol field ignored
    state.retain(|k, _| !(k.version == key.version && k.program == key.program));
    serialize_result(&true)
}

fn get_port(mapping: &Mapping) -> RequestResult {
    let state = STATE.read();
    let key = ProgramKey::from(mapping);
    let ret_val = match state.get(&key) {
        Some(val) => val.addr.port(),
        None => 0,
    };
    serialize_result(&u32::from(ret_val))
}

fn dump() -> RequestResult {
    let state = STATE.read();
    let mappings = state.iter().filter_map(|(key, description)| {
        let prot = key.portmapper_description()?;
        Some(Mapping {
            prog: key.program,
            vers: key.version,
            prot,
            port: description.addr.port().into(),
        })
    });
    let list = PMapList::create_list(mappings);

    match list {
        Some(list) => serialize_result(&[list]),
        None => serialize_result(&Option::<PMapList>::None),
    }
}

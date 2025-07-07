use std::{
    collections::hash_map::Entry,
    net::{Ipv4Addr, SocketAddrV4},
};

use onc_rpc::AcceptedStatus;
use rpcbind_rs::{
    request::PortMapperRequest,
    xdr_types::{
        CreateList,
        port_mapper::{Mapping, PMapList},
    },
};

use super::{RequestResult, serialize_result};
use crate::state::{ProgramDescription, ProgramKey, State};

pub fn process_request(request: &PortMapperRequest, state: &mut State) -> RequestResult {
    #[allow(unused_variables)]
    match request {
        PortMapperRequest::Null => Ok(Vec::new()),
        PortMapperRequest::Set(mapping) => set(state, mapping),
        PortMapperRequest::Unset(mapping) => unset(state, mapping),
        PortMapperRequest::GetPort(mapping) => get_port(state, mapping),
        PortMapperRequest::Dump => dump(state),
        PortMapperRequest::CallIt(call_args) => todo!(),
    }
}

fn set(state: &mut State, mapping: &Mapping) -> RequestResult {
    let key = ProgramKey::from(mapping);
    let port = mapping
        .port
        .try_into()
        .map_err(|_| AcceptedStatus::GarbageArgs)?;
    let val = ProgramDescription {
        addr: SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port),
        owner: None,
    };

    super::set(state, key, val)
}

fn unset(state: &mut State, mapping: &Mapping) -> RequestResult {
    let key = ProgramKey::from(mapping);
    // Protocol field ignored
    state.retain(|k, _| !(k.version == key.version && k.program == key.program));
    serialize_result(&true)
}

fn get_port(state: &State, mapping: &Mapping) -> RequestResult {
    let key = ProgramKey::from(mapping);
    let ret_val = match state.get(&key) {
        Some(val) => val.addr.port(),
        None => 0,
    };
    serialize_result(&u32::from(ret_val))
}

fn dump(state: &State) -> RequestResult {
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

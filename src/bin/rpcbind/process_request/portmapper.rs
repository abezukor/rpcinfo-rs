use rpcbind_rs::{
    request::PortMapperRequest,
    xdr_types::{
        CreateList,
        port_mapper::{Mapping, PMapList},
    },
};

use crate::state::State;

pub fn process_request(request: &PortMapperRequest, state: &mut State) -> Vec<u8> {
    #[allow(unused_variables)]
    match request {
        PortMapperRequest::Null => todo!(),
        PortMapperRequest::Set(mapping) => todo!(),
        PortMapperRequest::Unset(mapping) => todo!(),
        PortMapperRequest::GetPort(mapping) => todo!(),
        PortMapperRequest::Dump => dump(state),
        PortMapperRequest::CallIt(call_args) => todo!(),
    }
}

fn dump(state: &State) -> Vec<u8> {
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
        Some(list) => facet_xdr::to_vec(&[list]).unwrap(),
        None => facet_xdr::to_vec(&Option::<PMapList>::None).unwrap(),
    }
}

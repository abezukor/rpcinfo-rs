use rpcbind_rs::{
    request::RpcBindRequest,
    xdr_types::{
        CreateList,
        rpcbind::{RPCB, RPList},
    },
};

use crate::state::{ProgramKey, State, make_rpcb};

pub fn process_request(request: &RpcBindRequest, state: &mut State) -> Vec<u8> {
    #[allow(unused_variables)]
    match request {
        RpcBindRequest::Set(rpcb) => todo!(),
        RpcBindRequest::Unset(rpcb) => todo!(),
        RpcBindRequest::GetAddr(rpcb) => get_addr(rpcb, state),
        RpcBindRequest::Dump => dump(state),
        RpcBindRequest::GetTime => todo!(),
        RpcBindRequest::UADDR2TADDR(_) => todo!(),
        RpcBindRequest::TADDR2UADDR(netbuf) => todo!(),
        RpcBindRequest::GETVERSADDR(rpcb) => todo!(),
        RpcBindRequest::Indirect(rpcb_rmtcallargs) => todo!(),
        RpcBindRequest::GetAddrList(rpcb) => todo!(),
        RpcBindRequest::GetStat => todo!(),
    }
}

fn get_addr(rpcb: &RPCB, state: &State) -> Vec<u8> {
    let key = ProgramKey::from(rpcb);
    let entry = state.get(&key).unwrap();
    facet_xdr::to_vec(&entry.universal_address()).unwrap()
}

fn dump(state: &State) -> Vec<u8> {
    let list = RPList::create_list(state.iter().map(make_rpcb));

    match list {
        Some(list) => facet_xdr::to_vec(&[list]).unwrap(),
        None => facet_xdr::to_vec(&Option::<RPList>::None).unwrap(),
    }
}

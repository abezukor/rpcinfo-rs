use std::time::{SystemTime, UNIX_EPOCH};

use onc_rpc::AcceptedStatus;
use rpcbind_rs::{
    request::RpcBindRequest,
    xdr_types::{
        CreateList,
        rpcbind::{RPCB, RPList},
    },
};

use super::{RequestResult, decode_universal_address, serialize_result};
use crate::state::{ProgramDescription, ProgramKey, State, make_rpcb};

pub fn process_request(request: &RpcBindRequest, state: &mut State) -> RequestResult {
    #[allow(unused_variables)]
    match request {
        RpcBindRequest::Set(rpcb) => set(state, rpcb),
        RpcBindRequest::Unset(rpcb) => unset(state, rpcb),
        RpcBindRequest::GetAddr(rpcb) => get_addr(rpcb, state),
        RpcBindRequest::Dump => dump(state),
        RpcBindRequest::GetTime => get_time(),
        RpcBindRequest::UADDR2TADDR(_) => todo!(),
        RpcBindRequest::TADDR2UADDR(netbuf) => todo!(),
        RpcBindRequest::GETVERSADDR(rpcb) => todo!(),
        RpcBindRequest::Indirect(rpcb_rmtcallargs) => todo!(),
        RpcBindRequest::GetAddrList(rpcb) => todo!(),
        RpcBindRequest::GetStat => {
            // This call seems really annouing to do and a minor security risk
            Err(AcceptedStatus::ProcedureUnavailable)
        }
    }
}

fn set(state: &mut State, rpcb: &RPCB) -> RequestResult {
    let key = ProgramKey::from(rpcb);
    let val = ProgramDescription {
        addr: decode_universal_address(&rpcb.r_addr).map_err(rpcbind_rs::accepted_status_error)?,
        owner: (!rpcb.r_owner.is_empty()).then(|| rpcb.r_owner.clone()),
    };
    super::set(state, key, val)
}

fn unset(state: &mut State, rpcb: &RPCB) -> RequestResult {
    let removed = if rpcb.r_netid.is_empty() {
        let original_length = state.len();
        state.retain(|k, _| !(rpcb.r_prog == k.program && rpcb.r_vers == k.version));
        state.len() < original_length
    } else {
        let key = ProgramKey::from(rpcb);
        state.remove(&key).is_some()
    };
    serialize_result(&removed)
}

fn get_addr(rpcb: &RPCB, state: &State) -> RequestResult {
    let key = ProgramKey::from(rpcb);
    serialize_result(&match state.get(&key) {
        Some(entry) => entry.universal_address(),
        None => String::new(),
    })
}

fn dump(state: &State) -> RequestResult {
    let list = RPList::create_list(state.iter().map(make_rpcb));

    match list {
        Some(list) => serialize_result(&[list]),
        None => serialize_result(&Option::<RPList>::None),
    }
}

fn get_time() -> RequestResult {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AcceptedStatus::SystemError)?
        .as_secs();
    // RPCBIND seems subject to the 2038 bug
    serialize_result(&u32::try_from(since_epoch).map_err(|_| AcceptedStatus::SystemError)?)
}

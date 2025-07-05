use rpcbind_rs::request::RpcRequest;

use crate::state::State;

mod portmapper;
mod rpcbind;

pub fn process_request(request: &RpcRequest, state: &mut State) -> Vec<u8> {
    match request {
        RpcRequest::V2(port_mapper_request) => {
            portmapper::process_request(port_mapper_request, state)
        }
        RpcRequest::V3(rpc_bind_request) | RpcRequest::V4(rpc_bind_request) => {
            rpcbind::process_request(rpc_bind_request, state)
        }
    }
}

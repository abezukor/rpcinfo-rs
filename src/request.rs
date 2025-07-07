use bytes::Bytes;
use facet::Facet;
use onc_rpc::{AcceptedStatus, CallBody};

mod port_mapper;
mod rpcbind;

pub use port_mapper::PortMapperRequest;
pub use rpcbind::RpcBindRequest;

use crate::RpcBindResult;

#[derive(Debug)]
pub enum RpcRequest {
    V2(PortMapperRequest),
    // V4 Is backwa5rds compatible with V3 Requests
    V3(RpcBindRequest),
    V4(RpcBindRequest),
}

impl RpcRequest {
    pub fn from_body<T: AsRef<[u8]>>(value: &CallBody<T, Bytes>) -> RpcBindResult<Self> {
        Ok(match value.program_version() {
            2 => Self::V2(PortMapperRequest::from_body(value)?),
            3 => Self::V3(RpcBindRequest::from_body(value)?),
            4 => Self::V4(RpcBindRequest::from_body(value)?),
            _ => return Err(AcceptedStatus::ProgramMismatch { low: 2, high: 4 }),
        })
    }
}

fn deserialize_payload<'f, T: Facet<'f>, P: AsRef<[u8]>>(payload: P) -> RpcBindResult<T> {
    facet_xdr::deserialize(payload.as_ref()).map_err(|_| AcceptedStatus::GarbageArgs)
}

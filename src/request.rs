use bytes::Bytes;
use onc_rpc::CallBody;

use crate::error::DecodeError;

mod port_mapper;
mod rpcbind;

pub use port_mapper::PortMapperRequest;
pub use rpcbind::RpcBindRequest;

#[derive(Debug)]
pub enum RpcRequest {
    V2(PortMapperRequest),
    // V4 Is backwa5rds compatible with V3 Requests
    V3(RpcBindRequest),
    V4(RpcBindRequest),
}

impl RpcRequest {
    pub fn from_body<T: AsRef<[u8]>>(value: &CallBody<T, Bytes>) -> Result<Self, DecodeError> {
        Ok(match value.program_version() {
            2 => Self::V2(PortMapperRequest::from_body(value)?),
            3 => Self::V3(RpcBindRequest::from_body(value)?),
            4 => Self::V4(RpcBindRequest::from_body(value)?),
            invalid => return Err(DecodeError::UnknownProgramVersion(invalid)),
        })
    }
}

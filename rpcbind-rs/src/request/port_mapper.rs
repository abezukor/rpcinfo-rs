use onc_rpc::{AcceptedStatus, CallBody};

use super::deserialize_payload;
use crate::{
    RpcBindResult,
    xdr_types::port_mapper::{CallArgs, Mapping},
};

#[derive(Debug)]
pub enum PortMapperRequest {
    Null,
    Set(Mapping),
    Unset(Mapping),
    GetPort(Mapping),
    Dump,
    CallIt(CallArgs),
}

impl PortMapperRequest {
    pub fn from_body(value: &CallBody<impl AsRef<[u8]>, impl AsRef<[u8]>>) -> RpcBindResult<Self> {
        Ok(match value.procedure() {
            0 => Self::Null,
            1 => Self::Set(deserialize_payload(value.payload())?),
            2 => Self::Unset(deserialize_payload(value.payload())?),
            3 => Self::GetPort(deserialize_payload(value.payload())?),
            4 => Self::Dump,
            5 => Self::CallIt(deserialize_payload(value.payload())?),
            _ => {
                return Err(AcceptedStatus::ProcedureUnavailable);
            }
        })
    }
}

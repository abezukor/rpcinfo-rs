use bytes::Bytes;
use onc_rpc::CallBody;

use crate::{
    error::DecodeError,
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
    pub fn from_body<T: AsRef<[u8]>>(value: &CallBody<T, Bytes>) -> Result<Self, DecodeError> {
        Ok(match value.procedure() {
            0 => Self::Null,
            1 => Self::Set(facet_xdr::deserialize(value.payload())?),
            2 => Self::Unset(facet_xdr::deserialize(value.payload())?),
            3 => Self::GetPort(facet_xdr::deserialize(value.payload())?),
            4 => Self::Dump,
            5 => Self::CallIt(facet_xdr::deserialize(value.payload())?),
            invalid => {
                return Err(DecodeError::UnknownProcedure(invalid));
            }
        })
    }
}

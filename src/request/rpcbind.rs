use bytes::Bytes;
use onc_rpc::CallBody;

use crate::{
    error::{self, DecodeError},
    xdr_types::rpcbind::{NetBuf, RPCB, RmtCallArgs},
};

#[derive(Debug)]
pub enum RpcBindRequest {
    Set(RPCB),
    Unset(RPCB),
    GetAddr(RPCB),
    Dump,
    GetTime,
    UADDR2TADDR(String),
    TADDR2UADDR(NetBuf),
    GETVERSADDR(RPCB),
    Indirect(RmtCallArgs),
    GetAddrList(RPCB),
    GetStat,
}

impl RpcBindRequest {
    pub fn from_body<T: AsRef<[u8]>>(
        value: &CallBody<T, Bytes>,
    ) -> Result<Self, error::DecodeError> {
        Ok(match value.procedure() {
            1 => Self::Set(facet_xdr::deserialize(value.payload())?),
            2 => Self::Unset(facet_xdr::deserialize(value.payload())?),
            3 => Self::GetAddr(facet_xdr::deserialize(value.payload())?),
            4 => Self::Dump,
            /*
            //RPCBPROC_BCAST
            6 => Self::GetTime,
            7 => Self::UADDR2TADDR(str::from_utf8(&payload)?.to_owned()),
            8 => Self::TADDR2UADDR(NetBuf::try_from(payload)?),
            9 => Self::GETVERSADDR(RPCB::try_from(payload)?),
            10 => Self::Indirect(RmtCallArgs::try_from(payload)?),
            11 => Self::GetAddrList(RPCB::try_from(payload)?),
            */
            12 => Self::GetStat,
            val => {
                return Err(DecodeError::UnknownProcedure(val));
            }
        })
    }
}

/*
 * NOTE: RPCBPROC_BCAST has the same functionality as CALLIT;
 * the new name is intended to indicate that this
 * procedure should be used for broadcast RPC, and
 * RPCBPROC_INDIRECT should be used for indirect calls.
 */
/*
rpcb_rmtcallres
RPCBPROC_BCAST(rpcb_rmtcallargs) = RPCBPROC_CALLIT;
*/

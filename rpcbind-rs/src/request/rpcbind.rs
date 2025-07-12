use onc_rpc::{AcceptedStatus, CallBody};

use super::deserialize_payload;
use crate::{
    RpcBindResult,
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
    pub fn from_body(value: &CallBody<impl AsRef<[u8]>, impl AsRef<[u8]>>) -> RpcBindResult<Self> {
        Ok(match value.procedure() {
            1 => Self::Set(deserialize_payload(value.payload())?),
            2 => Self::Unset(deserialize_payload(value.payload())?),
            3 => Self::GetAddr(deserialize_payload(value.payload())?),
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
            _ => {
                return Err(AcceptedStatus::ProcedureUnavailable);
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

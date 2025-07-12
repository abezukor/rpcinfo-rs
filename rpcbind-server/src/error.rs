use std::fmt::{Debug, Display};

use onc_rpc::{
    AcceptedReply, AcceptedStatus, AuthError, RejectedReply, ReplyBody, auth::AuthFlavor,
};
use thiserror::Error;

pub type RPCResult<T> = Result<T, RPCError>;

pub type AcceptedStatusError = AcceptedStatus<[u8; 0]>;

#[derive(Debug, Error)]
pub struct RPCError(ReplyBody<[u8; 0], [u8; 0]>);

impl From<AuthError> for RPCError {
    fn from(value: AuthError) -> Self {
        Self(ReplyBody::Denied(RejectedReply::AuthError(value)))
    }
}

impl<P: AsRef<[u8]>> From<AcceptedStatus<P>> for RPCError {
    fn from(to_convert: AcceptedStatus<P>) -> Self {
        Self(ReplyBody::Accepted(AcceptedReply::new(
            AuthFlavor::AuthNone(None),
            convert_accepted_status(&to_convert),
        )))
    }
}

impl Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl<T: AsRef<[u8]>, P: AsRef<[u8]>> From<RPCError> for ReplyBody<T, P> {
    fn from(value: RPCError) -> Self {
        match value.0 {
            ReplyBody::Accepted(accepted_reply) => ReplyBody::Accepted(AcceptedReply::new(
                AuthFlavor::AuthNone(None),
                convert_accepted_status(accepted_reply.status()),
            )),
            ReplyBody::Denied(rejected_reply) => ReplyBody::Denied(rejected_reply),
        }
    }
}

fn convert_accepted_status<P: AsRef<[u8]>>(
    from: &AcceptedStatus<impl AsRef<[u8]>>,
) -> AcceptedStatus<P> {
    match from {
        AcceptedStatus::Success(_) => panic!("Converting a non_error Accepted Status"),
        AcceptedStatus::ProgramUnavailable => AcceptedStatus::ProgramUnavailable,
        AcceptedStatus::ProgramMismatch { low, high } => AcceptedStatus::ProgramMismatch {
            low: *low,
            high: *high,
        },
        AcceptedStatus::ProcedureUnavailable => AcceptedStatus::ProcedureUnavailable,
        AcceptedStatus::GarbageArgs => AcceptedStatus::GarbageArgs,
        AcceptedStatus::SystemError => AcceptedStatus::SystemError,
    }
}

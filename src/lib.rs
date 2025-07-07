use onc_rpc::AcceptedStatus;

pub mod xdr_types;

pub mod request;

pub type RpcBindResult<T> = Result<T, onc_rpc::AcceptedStatus<[u8; 0]>>;

/// panics if accepted status is a success
pub fn accepted_status_error<I: AsRef<[u8]>, O: AsRef<[u8]>>(
    to_convert: AcceptedStatus<I>,
) -> AcceptedStatus<O> {
    match to_convert {
        AcceptedStatus::Success(_) => panic!("Converting a non_error Accepted Status"),
        AcceptedStatus::ProgramUnavailable => AcceptedStatus::ProgramUnavailable,
        AcceptedStatus::ProgramMismatch { low, high } => {
            AcceptedStatus::ProgramMismatch { low, high }
        }
        AcceptedStatus::ProcedureUnavailable => AcceptedStatus::ProcedureUnavailable,
        AcceptedStatus::GarbageArgs => AcceptedStatus::GarbageArgs,
        AcceptedStatus::SystemError => AcceptedStatus::SystemError,
    }
}

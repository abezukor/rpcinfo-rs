pub mod xdr_types;

pub mod request;

pub type RpcBindResult<T> = Result<T, onc_rpc::AcceptedStatus<[u8; 0]>>;

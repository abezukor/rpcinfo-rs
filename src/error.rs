use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Error Deserailizing Request {0}")]
    XDRDeserialization(#[from] facet_xdr::XdrDeserError),
    #[error("Unknown Procedure {0}")]
    UnknownProcedure(u32),
    #[error("Unknown Program Version {0}")]
    UnknownProgramVersion(u32),
}

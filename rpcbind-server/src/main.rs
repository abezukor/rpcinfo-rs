use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::LazyLock,
};

use anyhow::{Result, anyhow, bail};
use bytes::BytesMut;
use onc_rpc::{
    AcceptedReply, AcceptedStatus, CallBody, Error as RPCError, MessageType, ReplyBody, RpcMessage,
    auth::AuthFlavor,
};
use parking_lot::RwLock;
use rpcbind_rs::request::RpcRequest;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    error::RPCResult,
    process_request::process_request,
    state::{ProgramDescription, ProgramKey, State},
};

mod error;
mod netconfig;
mod process_request;
mod state;

const RPCBIND_PORT: u16 = 111;
const PROGRAM_ID: u32 = 100000;

pub static STATE: LazyLock<RwLock<State>> = LazyLock::new(|| {
    let addrs = nix::ifaddrs::getifaddrs()
        .unwrap()
        .filter_map(|addr| addr.address)
        .filter_map(|addr| addr.as_sockaddr_in().copied())
        .map(|addr| addr.ip());

    let mut state = State::new();

    for addr in addrs {
        for version in 2u32..5 {
            state.insert(
                ProgramKey {
                    program: PROGRAM_ID,
                    version,
                    net_id: "tcp".to_owned(),
                },
                ProgramDescription {
                    addr: SocketAddrV4::new(addr, 111),
                    owner: Some("rpcbind-rs".to_owned()),
                },
            );
        }
    }
    RwLock::new(state)
});

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, RPCBIND_PORT))
        .await
        .unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("Error handling client {e:?}");
            }
        });
    }
}

const MSG_HEADER_LEN: usize = 4;

pub async fn handle_client(mut stream: impl AsyncRead + AsyncWrite + Unpin) -> Result<()> {
    println!("Got stream");

    let mut request_header = [0u8; MSG_HEADER_LEN];
    //read header
    stream.read_exact(&mut request_header).await?;
    let expected_len = get_length(&request_header)?;
    let mut message = BytesMut::from(request_header.as_slice());
    message.resize(expected_len, 0);

    stream.read_exact(&mut message[MSG_HEADER_LEN..]).await?;
    let message = match RpcMessage::try_from(message.freeze()) {
        Ok(message) => message,
        Err(RPCError::IncompleteHeader) => {
            unreachable!("MSG_HEADER_LEN {} is incorrect", MSG_HEADER_LEN)
        }
        Err(RPCError::IncompleteMessage {
            buffer_len,
            expected,
        }) => {
            unreachable!(
                "Message is still incomplete len {} expected {}",
                buffer_len, expected
            )
        }
        Err(e) => {
            bail!("Got another error when decoding header {:?}", e);
        }
    };

    println!("Message {message:?}");
    let xid = message.xid();

    let rpc_request = message
        .call_body()
        .ok_or_else(|| anyhow!("Server got response packet"))?;
    let body = match handle_request(rpc_request) {
        Ok(status) => ReplyBody::Accepted(AcceptedReply::new(
            AuthFlavor::<Vec<u8>>::AuthNone(None),
            status,
        )),
        Err(e) => e.into(),
    };

    let reply = RpcMessage::new(xid, MessageType::Reply(body));
    //reply.serialise_into(stream).unwrap();
    let reply = reply.serialise()?;
    stream.write_all(&reply).await?;

    Ok(())
}

fn handle_request(
    body: &CallBody<impl AsRef<[u8]>, impl AsRef<[u8]>>,
) -> RPCResult<AcceptedStatus<Vec<u8>>> {
    let request = RpcRequest::from_body(body)?;
    let return_value = process_request(&request)?;
    Ok(AcceptedStatus::Success(return_value))
}

fn get_length(header_bytes: &[u8; MSG_HEADER_LEN]) -> Result<usize> {
    match RpcMessage::try_from(header_bytes.as_slice()) {
        Ok(message) => {
            bail!(
                "We have only read the header so receiving {:?} should have been impossible",
                message
            );
        }
        Err(RPCError::IncompleteHeader) => {
            unreachable!("MSG_HEADER_LEN {} is incorrect", MSG_HEADER_LEN)
        }
        Err(RPCError::IncompleteMessage {
            buffer_len: _,
            expected,
        }) => Ok(expected),
        Err(e) => {
            bail!("Got another error when decoding header {:?}", e);
        }
    }
}

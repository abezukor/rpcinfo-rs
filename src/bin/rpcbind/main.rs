use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

use anyhow::{Result, anyhow, bail};
use bytes::BytesMut;
use onc_rpc::{
    AcceptedReply, AcceptedStatus, Error as RPCError, MessageType, ReplyBody, RpcMessage,
};
use rpcbind_rs::request::RpcRequest;

use crate::{
    process_request::process_request,
    state::{ProgramDescription, ProgramKey, State},
};

mod netconfig;
mod process_request;
mod state;

const RPCBIND_PORT: u16 = 111;
const PROGRAM_ID: u32 = 100000;

pub fn main() {
    let listener =
        TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, RPCBIND_PORT)).unwrap();

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

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, &mut state) {
                    eprintln!("Error handling client {e:?}");
                }
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {e}");
            }
        }
    }
}

const MSG_HEADER_LEN: usize = 4;

pub fn handle_client(mut stream: impl Read + Write, state: &mut State) -> Result<()> {
    println!("Got stream");

    let mut request_header = [0u8; MSG_HEADER_LEN];
    //read header
    stream.read_exact(&mut request_header).unwrap();
    let expected_len = get_length(&request_header)?;
    let mut message = BytesMut::from(request_header.as_slice());
    message.resize(expected_len, 0);

    stream.read_exact(&mut message[MSG_HEADER_LEN..]).unwrap();
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
    let auth_flavor = rpc_request.auth_verifier();
    let rpc_request = RpcRequest::from_body(rpc_request).unwrap();
    println!("Request {rpc_request:?}");

    let return_value = process_request(&rpc_request, state);
    let reply = RpcMessage::new(
        xid,
        MessageType::Reply(ReplyBody::Accepted(AcceptedReply::new(
            auth_flavor.clone(),
            AcceptedStatus::Success(return_value),
        ))),
    );
    //reply.serialise_into(stream).unwrap();
    let reply = reply.serialise().unwrap();
    stream.write_all(&reply).unwrap();

    Ok(())
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

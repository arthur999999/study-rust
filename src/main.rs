use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

use serde::{Deserialize, Serialize};
use solana_gossip::{
    cluster_info,
    contact_info::{self, ContactInfo},
    crds_gossip_pull::CrdsFilter,
    crds_value::{CrdsData, CrdsValue},
    ping_pong::{self, Ping, Pong},
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    timing::timestamp,
};
use solana_streamer::socket::SocketAddrSpace;

//dont work

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8001")?;
    println!("Socket UDP criado e vinculado a: {}", socket.local_addr()?);

    let my_ip: SocketAddr = "170.39.119.105:8001".parse().expect("Failed create my ip");

    let solana_addr: SocketAddr = "35.203.170.30:8001"
        .parse()
        .expect("Failed create socket testnet");

    let keypair = Keypair::new();

    let mut contact_info = ContactInfo::new(keypair.pubkey(), timestamp(), 0);

    let value = CrdsValue::new_signed(CrdsData::ContactInfo(contact_info), &keypair);

    let ping_message = Ping::new([2_u8; 32], &keypair).expect("failed creat ping");

    let message = Protocol::PingMessage(ping_message);

    // let message = Protocol::PullRequest(filter, value);

    let serealized = bincode::serialize(&message).expect("Failed bincode");

    let result_send = socket.send_to(&serealized, solana_addr);

    println!("result send {:?}", result_send);

    let recive_message = listen_for_gossip_messages(&socket);

    match recive_message {
        Some(message) => {
            let protocol: Protocol = bincode::deserialize(&message).expect("Failed deserialize");

            println!("Protocol {:?}", protocol);
        }
        None => {
            println!("No message recived");
        }
    }

    Ok(())
}

fn listen_for_gossip_messages(socket: &UdpSocket) -> Option<Vec<u8>> {
    let mut buf = [0u8; 1260];

    match socket.recv_from(&mut buf) {
        Ok((size, _src)) => {
            println!("message recived {:?}", buf);
            println!("message size {:?}", size);
            return Some(buf[..size].to_vec());
        }
        Err(e) => {
            eprintln!("Failed to receive gossip message: {}", e);
            return None;
        }
    }
}

const GOSSIP_PING_TOKEN_SIZE: usize = 32;

type PingType = ping_pong::Ping<[u8; GOSSIP_PING_TOKEN_SIZE]>;

#[derive(Debug, Deserialize, Serialize)]
pub enum Protocol {
    /// Gossip protocol messages
    PullRequest(CrdsFilter, CrdsValue),
    PullResponse(Pubkey, Vec<CrdsValue>),
    PushMessage(Pubkey, Vec<CrdsValue>),
    // TODO: Remove the redundant outer pubkey here,
    // and use the inner PruneData.pubkey instead.
    PruneMessage(Pubkey, PruneData),
    PingMessage(PingType),
    PongMessage(Pong),
    // Update count_packets_received if new variants are added here.
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct PruneData {
    /// Pubkey of the node that sent this prune data
    pubkey: Pubkey,
    /// Pubkeys of nodes that should be pruned
    prunes: Vec<Pubkey>,
    /// Signature of this Prune Message
    signature: Signature,
    /// The Pubkey of the intended node/destination for this message
    destination: Pubkey,
    /// Wallclock of the node that generated this message
    wallclock: u64,
}

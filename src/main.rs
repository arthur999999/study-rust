use std::{
    io,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use solana_bloom::bloom::Bloom;
use solana_gossip::{
    cluster_info,
    contact_info::{self, ContactInfo},
    crds_value::{CrdsData, CrdsValue},
    ping_pong::{self, Ping, Pong},
};
use solana_sdk::{
    hash::Hash,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    timing::timestamp,
};
use solana_streamer::socket::SocketAddrSpace;
use tokio::time::sleep;

//dont work

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8001").expect("Failed create UdpSocket");
    println!(
        "Socket UDP criado e vinculado a: {}",
        socket.local_addr().expect("failed local ip")
    );

    let my_ip: SocketAddr = "170.39.119.105:8001".parse().expect("Failed create my ip");

    let solana_addr: SocketAddr = "35.197.53.105:8001"
        .parse()
        .expect("Failed create socket testnet");

    let keypair = Keypair::from_base58_string(
        "378KQrAsYzWLYAmhZ2s5GqFhvLx4VuH9U7WKPKp2hqiUfP3fk6gf5dYwEuThtCDx7FJ6EqriPWuAXwxPFA1q1vP9",
    );

    let mut contact_info = ContactInfo::new(keypair.pubkey(), timestamp(), 0);
    contact_info.set_gossip(my_ip);

    let value = CrdsValue::new_signed(CrdsData::ContactInfo(contact_info), &keypair);

    let socket_clone = socket.try_clone().expect("Failed Clone socket");

    let keypair_clone = keypair.insecure_clone();

    tokio::spawn(async move {
        handle_ping(&keypair_clone, &socket_clone, solana_addr).await;
    });

    send_pong(&socket, &keypair);
}

async fn handle_ping(keypair: &Keypair, socket: &UdpSocket, solana_addr: SocketAddr) {
    let ping_message = Ping::new([2_u8; 32], keypair).expect("failed creat ping");

    let message = Protocol::PingMessage(ping_message);

    let serealized = bincode::serialize(&message).expect("Failed bincode");

    loop {
        let result_send = socket.send_to(&serealized, solana_addr);
        println!("Ping Sent {:?}", result_send);
        sleep(Duration::from_secs(20)).await;
    }
}

fn send_pong(socket: &UdpSocket, keypair: &Keypair) {
    loop {
        let protocol = listen_for_gossip_messages(socket);
        match protocol {
            Some((message, src)) => {
                let protocol: Protocol =
                    bincode::deserialize(&message).expect("Failed deserialize");

                println!("Message Received {:?}", protocol);
                match protocol {
                    Protocol::PullRequest(crds_filter, crds_value) => (),
                    Protocol::PullResponse(pubkey, vec) => (),
                    Protocol::PushMessage(pubkey, vec) => (),
                    Protocol::PruneMessage(pubkey, prune_data) => (),
                    Protocol::PingMessage(ping) => {
                        let pong = Pong::new(&ping, keypair).expect("Failed create pong");
                        let serealized = bincode::serialize(&Protocol::PongMessage(pong))
                            .expect("Failed bincode");

                        let result_send = socket.send_to(&serealized, src);

                        println!("pong sent {:?}", result_send);
                    }
                    Protocol::PongMessage(pong) => (),
                }
            }
            None => (),
        }
    }
}

fn listen_for_gossip_messages(socket: &UdpSocket) -> Option<(Vec<u8>, SocketAddr)> {
    let mut buf = [0u8; 1260];

    match socket.recv_from(&mut buf) {
        Ok((size, src)) => {
            println!("message recived from {:?}", src);

            return Some((buf[..size].to_vec(), src));
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct CrdsFilter {
    pub filter: Bloom<Hash>,
    pub mask: u64,
    pub mask_bits: u32,
}

impl Default for CrdsFilter {
    fn default() -> Self {
        fn compute_mask(seed: u64, mask_bits: u32) -> u64 {
            assert!(seed <= 2u64.pow(mask_bits));
            let seed: u64 = seed.checked_shl(64 - mask_bits).unwrap_or(0x0);
            seed | (!0u64).checked_shr(mask_bits).unwrap_or(!0x0)
        }

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        fn mask_bits(num_items: f64, max_items: f64) -> u32 {
            // for small ratios this can result in a negative number, ensure it returns 0 instead
            ((num_items / max_items).log2().ceil()).max(0.0) as u32
        }

        let max_items: u32 = 1287;
        let num_items: u32 = 512;
        let false_rate: f64 = 0.1f64;
        let max_bits = 7424u32;
        let mask_bits = mask_bits(f64::from(num_items), f64::from(max_items));

        let bloom: Bloom<Hash> = Bloom::random(max_items as usize, false_rate, max_bits as usize);

        CrdsFilter {
            filter: bloom,
            mask: compute_mask(0_u64, mask_bits),
            mask_bits,
        }
    }
}

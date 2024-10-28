// use std::{
//     io,
//     net::{SocketAddr, UdpSocket},
// };

// use serde::{Deserialize, Serialize};
// use solana_gossip::{
//     cluster_info,
//     contact_info::{self, ContactInfo},
//     crds_gossip_pull::CrdsFilter,
//     crds_value::{CrdsData, CrdsValue},
//     ping_pong::{self, Pong},
// };
// use solana_sdk::{
//     pubkey::Pubkey,
//     signature::{Keypair, Signature},
//     signer::Signer,
// };
// use solana_streamer::socket::SocketAddrSpace;

// fn main() -> std::io::Result<()> {
//     let socket = UdpSocket::bind("0.0.0.0:0")?;
//     println!("Socket UDP criado e vinculado a: {}", socket.local_addr()?);

//     let my_ip: SocketAddr = "187.122.60.205:8000".parse().expect("Failed create my ip");

//     let solana_addr: SocketAddr = "35.203.170.30:8001"
//         .parse()
//         .expect("Failed create socket testnet");

//     let keypair = Keypair::new();

//     let contact_info = ContactInfo::new_gossip_entry_point(&my_ip);

//     let value = CrdsValue::new_signed(CrdsData::ContactInfo(contact_info), &keypair);

//     let socket_space = SocketAddrSpace::new(true);

//     let send_message = cluster_info::push_messages_to_peer(
//         vec![value],
//         keypair.pubkey(),
//         solana_addr,
//         &socket_space,
//     );

//     println!("send message result {:?}", send_message);

//     listen_for_gossip_messages(&socket);

//     Ok(())
// }

// fn listen_for_gossip_messages(socket: &UdpSocket) {
//     let mut buf = [0u8; 2000];
//     match socket.recv_from(&mut buf) {
//         Ok((size, _src)) => {
//             println!("message recived {:?}", buf);
//         }
//         Err(e) => {
//             eprintln!("Failed to receive gossip message: {}", e);
//         }
//     }
// }

// const GOSSIP_PING_TOKEN_SIZE: usize = 32;

// type Ping = ping_pong::Ping<[u8; GOSSIP_PING_TOKEN_SIZE]>;

// #[derive(Debug, Deserialize, Serialize)]
// pub enum Protocol {
//     /// Gossip protocol messages
//     PullRequest(CrdsFilter, CrdsValue),
//     PullResponse(Pubkey, Vec<CrdsValue>),
//     PushMessage(Pubkey, Vec<CrdsValue>),
//     // TODO: Remove the redundant outer pubkey here,
//     // and use the inner PruneData.pubkey instead.
//     PruneMessage(Pubkey, PruneData),
//     PingMessage(Ping),
//     PongMessage(Pong),
//     // Update count_packets_received if new variants are added here.
// }

// #[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
// pub struct PruneData {
//     /// Pubkey of the node that sent this prune data
//     pubkey: Pubkey,
//     /// Pubkeys of nodes that should be pruned
//     prunes: Vec<Pubkey>,
//     /// Signature of this Prune Message
//     signature: Signature,
//     /// The Pubkey of the intended node/destination for this message
//     destination: Pubkey,
//     /// Wallclock of the node that generated this message
//     wallclock: u64,
// }

use std::error::Error;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // Cria um socket UDP que escuta no IP local e em uma porta específica
    let local_socket = UdpSocket::bind("0.0.0.0:0")?;
    println!("Escutando no endereço: {}", local_socket.local_addr()?);

    // Define o endereço de destino (seu IP) para enviar a mensagem
    let dest_addr = "170.39.119.105:8080"; // porta para onde enviar

    // Mensagem a ser enviada
    let message = b"testte";

    // Envia a mensagem para o endereço de destino
    local_socket.send_to(message, dest_addr)?;
    println!("Mensagem enviada para {}", dest_addr);

    // Thread para receber mensagens
    thread::spawn(move || {
        let mut buf = [0; 1024]; // Buffer para receber dados

        loop {
            // Recebe dados
            match local_socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let received_msg = String::from_utf8_lossy(&buf[..amt]);
                    println!("Recebido de {}: {}", src, received_msg);
                }
                Err(e) => {
                    eprintln!("Erro ao receber: {}", e);
                    break;
                }
            }
        }
    });

    // Mantém o programa em execução para permitir receber mensagens
    loop {
        thread::sleep(Duration::from_secs(1)); // Espera um segundo
    }
}

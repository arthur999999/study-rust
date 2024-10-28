use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

use solana_gossip::{
    cluster_info,
    contact_info::{self, ContactInfo},
    crds_value::{CrdsData, CrdsValue},
};
use solana_sdk::{signature::Keypair, signer::Signer};
use solana_streamer::socket::SocketAddrSpace;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    println!("Socket UDP criado e vinculado a: {}", socket.local_addr()?);

    let solana_addr: SocketAddr = "35.203.170.30:8001"
        .parse()
        .expect("Failed create socket testnet");

    let keypair = Keypair::new();

    let contact_info = ContactInfo::new_with_socketaddr(&keypair.pubkey(), &socket.local_addr()?);

    let value = CrdsValue::new_signed(CrdsData::ContactInfo(contact_info), &keypair);

    let socket_space = SocketAddrSpace::new(true);

    let send_message = cluster_info::push_messages_to_peer(
        vec![value],
        keypair.pubkey(),
        solana_addr,
        &socket_space,
    );

    println!("send message result {:?}", send_message);

    let result_read = read_socket(&socket);

    println!("read result {:?}", result_read);

    Ok(())
}

fn read_socket(socket: &UdpSocket) -> io::Result<()> {
    let mut buf = [0; 2000]; // Buffer de 1024 bytes para receber dados

    // Recebe dados de um remetente e armazena no buffer
    let (amt, src) = socket.recv_from(&mut buf)?;
    println!("Dados recebidos de {}: {:?}", src, &buf[..amt]);

    // Opcional: Processa os dados recebidos
    let mensagem = String::from_utf8_lossy(&buf[..amt]);
    println!("Mensagem: {}", mensagem);

    Ok(())
}

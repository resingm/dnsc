use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;

use trust_dns_proto::serialize::binary::BinEncodable;

use crate::dns::build_query;

// use trust_dns_proto::op::{Message, MessageType, OpCode, Query};
// use trust_dns_proto::rr::domain::Name;
// use trust_dns_proto::rr::record_type::RecordType;

mod dns;


enum Message {
    Input(String),
    Terminate,
}


/// TODO: Documentation
/// 
fn read_input(channel: mpsc::Sender<Message>) {
    let stdin = io::stdin();
    
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("#") {
                continue;
            }

            channel.send(Message::Input(String::from(line))).expect("Failed to queue input to the channel.")
        }
    }

    channel.send(Message::Terminate).expect("Failed to queue `TERMINATE` message.");
}


/// TODO: Documentation
fn run_dns_query_tx(channel: mpsc::Receiver<Message>, socket: UdpSocket) {

    // let resolver_config = 
    // let dnsq = trust_dns_proto::op::Message::query()

    let resolver_addr = ("8.8.8.8", 53);
    let qid: u16 = 0;
    // TODO: Increment query ID with every new query.

    for msg in channel {
        match msg {
            Message::Input(line) => {
                let q = build_query(qid, &line).expect("Failed to build a query.");
                let query_bytes = q.to_vec().expect("Failed to serialize the query to bytes.");

                // Send the query to the resolver
                socket.send_to(&query_bytes, resolver_addr)
        .expect("Failed to send DNS query");
                println!("{}", line);
            }
            Message::Terminate => {
                break;
            }
        }
    };
}


/// TODO: Add some description
fn run_dns_query_rx(socket: UdpSocket) {

}


fn main() {

    let (tx, rx) = mpsc::channel();

    let socket = UdpSocket::bind("0.0.0.0:53535").expect("Failed to bind UDP socket.");
    let socket_tx = socket.try_clone().expect("Failed to clone UDP socket for lookup thread.");
    let socket_rx = socket.try_clone().expect("Failed to clone UDP socket for listener thread.");

    // Spawn the DNS query consumer

    let thread_input_reader= thread::spawn(move || {
        read_input(tx);
    });

    let thread_dns_query_tx= thread::spawn(move || {
        run_dns_query_tx(rx, socket_tx);
    });

    let thread_dns_query_rx = thread::spawn(move || {

    });

    thread_input_reader.join().expect("Failed to terminate input reader.");
    thread_dns_query_tx.join().expect("Failed to terminate DNS query sender.");
    thread_dns_query_rx.join().expect("Failed to terminate DNS query receiver.");
}

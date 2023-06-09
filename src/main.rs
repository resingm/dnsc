use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use clap::Parser;
use trust_dns_proto::rr::record_type;

mod cli;
mod dns;
mod util;


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
}


/// TODO: Documentation
fn run_dns_query_tx(
    channel: mpsc::Receiver<Message>,
    socket: UdpSocket,
    nameserver: &str,
    nameserver_port: u16,
    rate_limit: u64,
    q_type: record_type::RecordType,
    // q_channel: mpsc::Sender<u16>,
) {
    let resolver_addr = (nameserver, nameserver_port);
    // TODO: Start with a random query ID
    let mut qid: u16 = 0;

    let wait_time = if rate_limit > 0 {
        (Duration::new(1, 0).as_millis() / rate_limit as u128) as u64
    } else {
        0
    };
    let wait_time = Duration::from_millis(wait_time);


    for msg in channel {
        qid += 1;

        match msg {
            Message::Input(line) => {
                let q = dns::build_query(qid, &line, q_type.clone()).expect("Failed to build a query.");
                let query_bytes = q.to_vec().expect("Failed to serialize the query to bytes.");

                // Send the query to the resolver
                socket.send_to(&query_bytes, resolver_addr).expect("Failed to send DNS query");

                if rate_limit > 0 {
                    thread::sleep(wait_time);
                };
            }
            Message::Terminate => {
                break;
            }
        }
    };
}


/// TODO: Add some description
fn run_dns_query_rx(
    socket: UdpSocket,
    timeout: u64,
    // q_channel: mpsc::Receiver<u16>,
) {
    let mut buffer = [0; 4096];
    socket.set_read_timeout(
        Some(Duration::new(timeout, 0))
    ).expect("Failed to set timeout on the DNS receiver socket.");

    loop {
        match socket.recv_from(&mut buffer) {

            Ok((received, src_addr)) => {
                // Process the received DNS query
                let response_data = &buffer[..received];
                let r = dns::parse_query(response_data).expect("Failed to parse a DNS response.");
                dns::response_to_csv(src_addr, r);
            }
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                break;
            }
            Err(err) => {
                // Handle the receive error
                util::err(&format!("Receive error: {:?}", err));
                break;
            }
        }
    }
}


fn main() {
    // Parse args
    let args = cli::ArgParse::parse();

    let ns= args.nameserver;
    let ns_port = args.port;
    let timeout = args.timeout;
    let bind = format!("{}:0", args.bind);
    let rate_limit = args.ratelimit;
    let with_header = args.no_header;
    let q_type = args.qtype;
    let q_type = cli::parse_record_type(&q_type).expect("Failed to parse query type.");


    let (tx, rx) = mpsc::channel();
    // let (q_tx, q_rx) = mpsc::channel();

    // The 0.0.0.0:0 binds to an ephemeral port
    let socket = UdpSocket::bind(bind).expect("Failed to bind UDP socket.");
    let socket_tx = socket.try_clone().expect("Failed to clone UDP socket for lookup thread.");
    let socket_rx = socket.try_clone().expect("Failed to clone UDP socket for listener thread.");

    // Print the CSV header
    if with_header {
        util::log(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            "resolver", "port", "qname", "qtype", "qclass", "rcode", "rname", "rtype", "rclass", "ttl", "rdata",
        ));
    }

    // Spawn the DNS query consumer
    let tx_input_reader = tx.clone();
    let thread_input_reader= thread::spawn(move || {
        read_input(tx_input_reader);
    });

    let thread_dns_query_tx= thread::spawn(move || {
        // run_dns_query_tx(rx, socket_tx, q_tx);
        run_dns_query_tx(rx, socket_tx, &ns, ns_port, rate_limit, q_type);
    });

    let thread_dns_query_rx = thread::spawn(move || {
        // run_dns_query_rx(socket, q_rx);
        run_dns_query_rx(socket_rx, timeout);
    });

    match thread_input_reader.join() {
        Ok(_) => (),
        Err(e) => util::err(&format!("Failed to terminate input reader: {:?}", e)),
    };

    match thread_dns_query_rx.join() {
        Ok(_) => (),
        Err(e) => util::err(&format!("Failed to terminate DNS query receiver: {:?}", e))
    };

    match tx.send(Message::Terminate) {
        Ok(_) => (),
        Err(e) => util::err(&format!("Failed to send the termination message to the DNS query sender: {:?}", e)),
    };

    match thread_dns_query_tx.join() {
        Ok(_) => (),
        Err(e) => util::err(&format!("Failed to terminate DNS query sender: {:?}", e)),
    };
}

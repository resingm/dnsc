use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(name = "dnsc")]
#[command(author = "Max Resing <contact@maxresing.de>")]
#[command(version = "0.1.0")]
#[command(about = "DNS resolver for massive DNS queries to a single DNS resolver.", long_about=None)]
pub struct ArgParse {
    pub nameserver: String,

    #[arg(short, long, required=false, default_value="53", help="Port of the recursive resolver")]
    pub port: u16,

    #[arg(short, long, required=false, default_value="3", help="Timeout to wait for responses before shutting down the listener")]
    pub timeout: u64,

    #[arg(short, long, required=false, default_value="0.0.0.0", help="IP address the listener should bind to")]
    pub bind: String,

    #[arg(short, long, required=false, default_value="0", help="Number of queries per second. 0 for no rate limit")]
    pub ratelimit: u64,

    #[arg(short, long, action=ArgAction::SetFalse, help="Output will have no column names printed on top")]
    pub no_header: bool,
}

pub fn print_csv_header() {
    println!(
        "{},{},{},{},{},{},{},{},{},{},{}",
        "resolver",
        "port",
        "qname",
        "qtype",
        "qclass",
        "rc",
        "rname",
        "rr",
        "rclass",
        "ttl",
        "rdata",
    )
}
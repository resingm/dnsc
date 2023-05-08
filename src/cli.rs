use clap::Parser;

#[derive(Parser)]
#[command(name = "dnsc")]
#[command(author = "Max Resing <contact@maxresing.de>")]
#[command(version = "0.1.0")]
#[command(about = "DNS resolver for massive DNS queries to a single DNS resolver.", long_about=None)]
pub struct ArgParse {
    pub nameserver: String,

    #[arg(short, long, required=false, default_value="53")]
    pub port: u16,

    #[arg(short, long, required=false, default_value="3")]
    pub timeout: u64,

    #[arg(short, long, required=false, default_value="0.0.0.0")]
    pub bind: String,
}
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The websocket address to connect to
    #[arg(short, long)]
    pub addr: String,

    /// How many concurrent sockets to use
    #[arg(short, long, default_value = "1")]
    pub connections: usize,

    /// How many messages send per socket
    #[arg(short, long, default_value = "1")]
    pub messages: usize,

    /// Message to send
    #[arg(default_value = "hello world!")]
    pub message: String,

    #[arg(short, long, default_value = "false")]
    pub log_errors: bool,
}

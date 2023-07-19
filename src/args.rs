use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The websocket address to connect to, e.g. ws://0.0.0.0:8080
    #[arg(short, long)]
    pub addr: String,

    /// How many concurrent sockets to open
    #[arg(short, long, default_value = "1")]
    pub connections: usize,

    /// How many messages send per 1 socket
    #[arg(short, long, default_value = "1")]
    pub messages: usize,

    /// Message to send
    #[arg(default_value = "hello world!")]
    pub message: String,

    /// Collect and print errors in the end of the tests
    #[arg(short = 'e', long, default_value = "false")]
    pub log_errors: bool,

    /// Show responses from the server with the specified sample rate [0-1]
    #[arg(short = 'r', long, default_value = "0.0", value_parser = parse_sample_rate)]
    pub resp_sample_rate: f64,
}

fn parse_sample_rate(src: &str) -> Result<f64, String> {
    let rate = src.parse::<f64>().map_err(|err| err.to_string())?;
    if !(0.0..=1.0).contains(&rate) {
        return Err(format!("Sample rate must be in range [0-1], got {}", rate));
    }
    Ok(rate)
}

mod args;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use clap::Parser;
use futures::stream::SplitStream;
use futures::{SinkExt, StreamExt};
use human_bytes::human_bytes;
use rand::Rng;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message::Text;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    let ok_counter = Arc::new(AtomicUsize::new(0));
    let addr = string_to_static_str(&args.addr);
    let barrier = Arc::new(tokio::sync::Barrier::new(args.connections + 1));
    let mut tasks = Vec::new();

    for _ in 0..args.connections {
        let ok_counter = ok_counter.clone();
        let msg = args.message.clone();
        let barrier = barrier.clone();
        let task = tokio::spawn(async move {
            let client_tungstenite = connect_async(addr).await.expect("Failed to connect").0;
            let (mut tx, rx) = client_tungstenite.split();

            read_responses(rx, args.resp_sample_rate);

            barrier.wait().await;

            // Async, quite fast, with feedback (send_all is faster but we can't count
            // messages)
            let mut map = hashbrown::HashMap::new();
            for _ in 0..args.messages {
                match tx.feed(Text(msg.clone())).await {
                    Ok(_) => {
                        ok_counter.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(err) => {
                        if args.log_errors {
                            let err = format!("{:?}", err);
                            if map.contains_key(&err) {
                                let count = map.get_mut(&err).unwrap();
                                *count += 1;
                            } else {
                                map.insert(err, 1);
                            }
                        }
                    }
                }
            }
            tx.flush().await.expect("flush failed");
            println!("Errors: {:?}", map);
        });
        tasks.push(task);
    }

    barrier.wait().await; // starts all tasks at the same time
    let start = Instant::now();

    println!(
        "Benchmarking: {} messages, {} connections",
        args.messages, args.connections
    );

    for task in tasks {
        task.await.unwrap();
    }

    let elapsed = Instant::now().duration_since(start).as_secs_f64();
    let ok_req = ok_counter.load(Ordering::Relaxed);
    let total_req = args.connections * args.messages;
    let rate = ok_req as f64 / elapsed;

    println!("Benchmark results:");
    println!("  Ok: {}", ok_req);
    println!("  Err: {}", total_req - ok_req);
    println!("  Total: {}", total_req);
    println!("  Elapsed time: {:.2} s", elapsed);
    println!("  Connections: {}", args.connections);
    println!("  Msg len: {}", human_bytes(args.message.len() as f64));
    println!("  Requests per second: {:.2} rps", rate);
    println!(
        "  Throughput: {:.2} Mbit/sec",
        rate * args.message.len() as f64 / 1_000.0 / 1000.0 * 8.0
    );
}

fn string_to_static_str(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

fn read_responses(
    mut rx: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    sample_rate: f64,
) {
    if sample_rate == 0.0 {
        return;
    }

    // read all messages from rx and write in into stdout
    tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            if sample_rate == 1.0 || rand::thread_rng().gen_bool(sample_rate) {
                let received = msg.expect("Failed to receive a message");
                if received.is_text() {
                    println!("> {}", received.into_text().unwrap());
                }
            };
        }
    });
}

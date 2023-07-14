mod args;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use clap::Parser;
use futures::SinkExt;
use human_bytes::human_bytes;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message::Text;

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
            let mut client_tungstenite = connect_async(addr).await.unwrap().0;
            barrier.wait().await;

            // Async, quite fast, with feedback (send_all is faster but we can't count
            // messages)
            for _ in 0..args.messages {
                if client_tungstenite.send(Text(msg.clone())).await.is_ok() {
                    ok_counter.fetch_add(1, Ordering::Relaxed);
                }
            }
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
    println!("  Connections: {}", args.messages);
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

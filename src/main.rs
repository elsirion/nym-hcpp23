use std::io::Write;
use nym_sdk::mixnet::{MixnetClient, MixnetMessageSender, Recipient};
use clap::Parser;

#[derive(Debug, Clone, Parser)]
enum Opts {
    Server,
    Client {
        recipient: Recipient,
    }
}

#[tokio::main]
async fn main() {
    let opts: Opts = Parser::parse();

    // For debugging uncomment the following line
    // nym_bin_common::logging::setup_logging();

    println!("Starting Nym client ...");
    let mut nym_client = MixnetClient::connect_new().await.expect("Could not build Nym client");

    match opts {
        Opts::Client { recipient } => {
            loop {
                print!("Write: ");
                std::io::stdout().flush().expect("Failed to flush stdout");
                let mut msg = String::new();
                std::io::stdin().read_line(&mut msg).expect("Stdin read failed");
                nym_client.send_plain_message(recipient, msg.trim()).await.expect("Nym send failed");
            }
        },
        Opts::Server => {
            println!("Nym address: {}", nym_client.nym_address());
            loop {
                let Some(msgs) = nym_client.wait_for_messages().await else { continue };
                for msg in msgs {
                    println!("Received: {}", String::from_utf8_lossy(&msg.message));
                }
            }
        }
    }
}

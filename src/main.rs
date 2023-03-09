mod client;

use std::thread::sleep;
use std::time;
use client::Client;

fn connect() -> Client {
    const DOMAINS: [&str; 1] = [
        "localhost:2130",
    ];

    const DOMAINS_LENGTH: usize = DOMAINS.len() + 1;

    let cli: Client;
    let mut n: usize = 0;
    loop {
        if n >= DOMAINS_LENGTH{
            n = 0
        }
        match Client::connect(DOMAINS[n]) {
            Ok(c) => {cli = c; break},
            _ => {}
        }
        n += 1;
        sleep(time::Duration::from_secs(60))
    }
    cli
}

fn main() {
    let cli = connect();
    cli.handle_packets();
}

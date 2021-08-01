use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

/// log just prints some stuff
///
/// Example usage:
/// ```rust
/// // outputs "[pog] someone is pogging!"
/// log("pog", "someone is pogging!");
/// ```
fn log(func: &str, msg: &str) {
    println!("[{}] {}", func, msg);
}

fn main() {
    let host = "127.0.0.1";
    let port = "1717";
    let host_and_port = host.to_owned() + ":" + port;
    let response = vec![
        "Buy Team Fortress classic, it's not free!\r\n",
        "Play Team Fortress 2, it's free!\r\n"];
    let mut i = 0;

    let listener = TcpListener::bind(&host_and_port).unwrap();
    log("TcpListener::bind", &*("Binding things on ".to_owned() + &host_and_port));

    log("listener.incoming", "now accepting new connections!");
    for stream in listener.incoming() {
        i = (i + 1) % response.len();
        let stream = stream.unwrap();
        handle_connection(stream, response[i]);
    }
}

fn handle_connection(mut stream: TcpStream, resp: &str) {
    log("handle_connection", "handling new connection");
    stream.write(resp.as_bytes()).unwrap();
    log("stream.write", "sending random quote of the day");
    stream.flush().unwrap();
    log("stream.flush", "done flushing");
}
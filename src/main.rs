use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Start listening...");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("New connection!");
    }
}

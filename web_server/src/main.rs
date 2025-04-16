use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    listener.incoming().for_each(|stream| {
        let _stream = stream.unwrap();

        println!("Connection setablished!")
    });
}

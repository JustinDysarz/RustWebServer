use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    ops::Index,
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    let pool = ThreadPool::new(8);

    listener
        .incoming()
        .for_each(|stream| pool.execute(|| handle_connection(stream.unwrap())));

    pool.drop();

    /*listener.incoming().for_each(|stream| {
        let stream = stream.unwrap();
        println!("Connection setablished!");
        handle_connection(stream);
    });*/
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|i| i.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let reqest_line = http_request.index(0);

    let (status_line, file) = match &reqest_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs_f64(5.));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "404.html"),
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);
}

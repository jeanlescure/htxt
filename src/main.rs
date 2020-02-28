use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream, txt: String) {
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body><pre>{}</pre></body></html>\r\n", txt);
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream, txt: String) {
    handle_read(&stream);
    handle_write(stream, txt);
}

fn handle_txt() -> String {
    let args: Vec<String> = env::args().collect();

    let mut txt = String::new();

    if args.len() > 1 {
        txt.push_str(&args[args.len()-1]);
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        while let Ok(n_bytes) = stdin.read_to_string(&mut txt) {
            if n_bytes == 0 { break }
        }
    }

    return txt;
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream, handle_txt())
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

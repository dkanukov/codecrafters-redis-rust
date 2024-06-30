use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_command_with_thread(mut stream: TcpStream) {
    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            let read_count = stream.read(&mut buffer).unwrap();
            println!("{}", read_count);

            if read_count == 0 {
                break;
            }
            stream.write_all(b"+PONG\r\n").unwrap();
        }
    });
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_command_with_thread(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

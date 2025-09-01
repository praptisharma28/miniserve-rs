use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap(); //returns result type
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        let contents  = fs::read_to_string("index.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        }
    else{
        let status_line= "HTTP/1.1 404 NOT FOUND";
        let contents  = fs::read_to_string("404.html").unwrap();
        let response = format!(
            "{}\r\nContent-length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        };

    }

//HTTP-Version status-code reason-phrase CRLF
//headers CRLF
//message body
//ex: HTTP/1.1 200 OK\r\n\r\n
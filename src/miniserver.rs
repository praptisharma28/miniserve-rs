use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;

/// A simple miniserver that listens on a specified address.
///
/// This server is designed to be lightweight and easy to use.
/// It can be extended with additional functionality as needed.
pub struct Miniserver {
    addr: String,
}

// let arr: [u8; 5] = [1, 2, 3, 4, 5];

impl Miniserver {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    // fn new here is an associated function of Miniserver

    pub fn run(self) {
        println!("Running server at {}", self.addr);
        //self in rust works as this in other languages, it holds the ownership of the struct
        // we will use self instead of &self because we want to take ownership of the struct
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    println!("Parsed request: {:?}", request);
                                    // Here you can handle the request further
                                }
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                            // let res: &Result<Request,  > = &buffer[..].try_into();
                            //these are the two ways we can call our new conversion function
                        }
                        Err(e) => println!("Failed to read from stream: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
            // let res = listener.accept();

            // if res.is_err(){
            //     continue;
            // }

            // let (stream, addr) = res.unwrap();
        }

        // let tup = (5, "a", listener);
    }
}

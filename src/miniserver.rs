use std::net::TcpListener;
use std::io::Read;

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
        }
        //self in rust works as this in other languages, it holds the ownership of the struct
        // we will use self instead of &self because we want to take ownership of the struct
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept(){
                Ok((stream, _)) => {
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer);
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

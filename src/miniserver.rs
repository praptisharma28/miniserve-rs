use std::net::TcpListener;

    /// A simple miniserver that listens on a specified address.
    ///
    /// This server is designed to be lightweight and easy to use.
    /// It can be extended with additional functionality as needed.
pub struct Miniserver {
        addr: String,
    }

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
        let listener = TcpListener::bind(&self.addr);
}

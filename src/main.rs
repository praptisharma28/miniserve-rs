fn main() {
    let miniserver = Miniserver::new("127.0.1:8080");//new is an associated function of Miniserver
    miniserver.run();

    // miniserver here is a struct
    struct Miniserver {
        addr: String,
    }

    impl Miniserver{
        fn new(addr: String) -> Self {
            Self {
                addr
            }
        }
        // fn new here is an associated function of Miniserver

        fn run(self) {
            println!("Running server at {}", self.addr);
        }
        //self in rust works as this in other languages, it holds the ownership of the struct
        // we will use self instead of &self because we want to take ownership of the struct
    }
}

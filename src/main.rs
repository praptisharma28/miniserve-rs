use std::option::Option;
use miniserver::Miniserver;
use http::request::Request;
fn main() {
    // let string = String::from("127.0.0.1:8080");//new is an associated function of Miniserver
    // let string_slice = &string[10..]; // this means give everything after the 10th byte
    // let string_borrow: &str = &string;
    // let string_literal = "1234";
    //normal string can shrink and explan dynamically at run time but slice strings are immutable

    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);

    let miniserver = Miniserver::new("127.0.0.1:8080".to_string());
    miniserver.run();
}
// miniserver here is a struct

mod miniserver {
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
    }
}

mod http {
    pub mod request {
        use super::method::Method;
        #[allow(dead_code)]
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }
    }
    pub mod method {
        pub enum Method {
            GET,
            POST,
            PUT,
            DELETE,
            PATCH,
            HEAD,
            OPTIONS,
            TRACE,
            CONNECT,
        }
    }
}

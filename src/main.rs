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

    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;
    // let head = Method::HEAD;
    // let patch = Method::PATCH;
    // let options = Method::OPTIONS;
    // let connect = Method::CONNECT;
    // let trace = Method::TRACE;

    let miniserver = Miniserver::new("127.0.0.1:8080".to_string());
    miniserver.run();
}
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

struct Request{
    path: String,
    query_string: String,
    method: String,
}

enum Method{
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
    OPTIONS,
    CONNECT,
    TRACE
}

// In memory, enums are numbers starting from 0 and incrementing by 1 for each variant
// enum for every variant holds the same memory size as the largest variant
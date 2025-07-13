use miniserver::Miniserver;
use http::Method;
use http::Request;

mod miniserver;
mod http;

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

# miniserve-rs
A minimal HTTP server built in Rust from scratch to explore low-level networking concepts, request parsing, and response handling. Inspired by how modern web servers work under the hood.

## Thoery
- Http/1.1
- it is a l7 protocol. 
- sent over tcp
- message based

- request: GET/search?name=abc HTTP/1.1 Accept:text/html
- response: HTTP/1.1 200 OK Content-Type: text/html 

- in the architecture, we have 3 main components:

tcp listener
http parser
handler

all of these will be running on a single thread

- String and &str
- string slicing helps in keeping memory lesser as we dont need to copy the whole string, think of string as something that stores length, capacity and ptr, while &str will store length and ptr

- the option enum:
rust doesnot support null values, but it still want to express, using enum 

pub enum Option<T> {
    None, 
    Some<T>
}

now we know how to express null values without the fear of null pointer exceptions

- the result enum

rust groups error in 2 categories recoverable and unrecoverable errors
rust doesnt support exceptions, instead it has result enum for handling recoverable errors

pub enum Result<T, E>{
    OK(T),
    Err(E),
}

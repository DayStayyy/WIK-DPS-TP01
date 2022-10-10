use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use rocket::serde::json::Json;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}



fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    // let Header = {}
    // let string Test = "";
    // for line in http_request {
    //     // split line with the first ":"
    //     let lineSplit: Vec<&str> = line.split(":").collect();
    //     Test += lineSplit[0] + " : " + lineSplit[1] + ",";
    // }


    println!("Request: {:#?}", Test);
}

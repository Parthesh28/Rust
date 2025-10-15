use::std::net::TcpListener;
use std::{ fs, io::{BufRead, BufReader, Write}, net::TcpStream};

fn main(){
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();


    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream){

        let buf_reader = BufReader::new(&stream);
        let req_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_code, filename) = if req_line == "GET / HTTP/1.1"{ ("HTTP/1.1 200 OK", "src/index.html")}
         else {("HTTP/1.1 404 NOT FOUND", "src/error.html")};
         


        let body  = fs::read_to_string(filename).unwrap();

        let length = body.len();

        let resp =  format!("{status_code}\r\nContent-Length: {length}\r\n\r\n{body}");

        stream.write_all(resp.as_bytes()).unwrap();

    }
}
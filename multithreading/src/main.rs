use::std::net::TcpListener;
use std::{fmt::format, fs, io::{BufRead, BufReader, Write}, net::TcpStream};

fn main(){
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();


    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream){

        let buf_reader = BufReader::new(&stream);
        let req_line = buf_reader.lines().next().unwrap().unwrap();

      if req_line == "GET / HTTP/1.1"{
          let status_code = "HTTP/1.1 200 OK";
          let body = fs::read_to_string("src/index.html").unwrap();
          let length = body.len();
          
          let res = format!("{status_code}\r\nContent-Length: {length}\r\n\r\n{body}");
          
          stream.write_all(res.as_bytes()).unwrap();
        } else{
            let status_code = "HTTP/1.1 404 NOT FOUND";
            let body = fs::read_to_string("src/error.html").unwrap();
            let length = body.len();
            
            let res = format!("{status_code}\r\nContent-Length: {length}\r\n\r\n{body}");
            stream.write_all(res.as_bytes()).unwrap();

      }  

    }
}
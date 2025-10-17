use::std::net::TcpListener;
use multithreading::ThreadPool;

fn multi(){
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

}
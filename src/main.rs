use std::io::{Read, Write};
use std::{fs, net::{SocketAddr, TcpListener, TcpStream}};
use std:: thread;

fn main() 
{
    let addr  = "0.0.0.0:6969";
    let listener : TcpListener = TcpListener::bind(addr).unwrap();
    loop 
    {
        accept_clients(&listener);
    }
}

fn accept_clients(listener : &TcpListener)
{
    match listener.accept()
    {
        Ok((stream, addr)) => 
        {
            thread::spawn(move || on_connected(stream, addr));
        },
        Err(e) => println!("Couldn't accept a client due to error {}", e)
    };
}

fn on_connected(mut stream : TcpStream, _addr : SocketAddr)
{
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("Received html:\n{:}", String::from_utf8_lossy(&buf));

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("Pages/page.html").unwrap();
    let length = contents.len();
    
    let response =
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    let count = stream.write(response.as_bytes()).unwrap();
    println!("Sent back html in {count} bytes");
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
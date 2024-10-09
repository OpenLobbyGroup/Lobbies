use std::{net::{Ipv4Addr, TcpListener}, ops::Add};
use std:: thread;

const OL_PORT : &str = "6969";

fn main() 
{
    let addr : String = Ipv4Addr::LOCALHOST.to_string().add(":").add(OL_PORT);
    let listener : TcpListener = TcpListener::bind(addr).unwrap();
    accept_clients(listener);
}

fn accept_clients(listener : TcpListener)
{
    match listener.accept() 
    {
        Ok((_socket, addr)) => thread::spawn(|| on_connected),
        Err(e) => panic!("{}", e)
    };
}

fn on_connected()
{
    println!("Client connected!");
}
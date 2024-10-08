use std::{net::{Ipv4Addr, TcpListener}, ops::Add};

const OL_PORT : &str = "6969";

fn main() 
{
    let addr : String = Ipv4Addr::LOCALHOST.to_string().add(":").add(OL_PORT);
    let listener : TcpListener = TcpListener::bind(addr).unwrap();
    accept_clients(listener);   
}

async fn accept_clients(listener : TcpListener)
{
    match listener.accept() 
    {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}")
    };
}
// use tokio::net::TcpListener;

// // wrap the async main function and let complier think this is a main function
// #[tokio::main] 
// async fn main() {
//     // tcp listener waiting for the incoming connects, await is for waiting
//     let listener = TcpListener::bind("localhost:8080").await.unwrap();

//     // accept the connection from tcp listener and yield the connection as well as the address of the connection
//     let (socket, _addr) = listener.accept().await.unwrap();
// }

use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    // create buffer
    let mut buffer = [0; 512];
    // save connection address
    let address = stream.peer_addr()?;
    // print out the connection
    println!("the address is {}", address);

    loop{
        // read the stream
        let read = stream.read(&mut buffer)?;
        // check the message is Ctrl + C or not
        if &buffer[..read] == [255, 244, 255, 253, 6]{
            // print out the exit message
            println!("{} received exit signal", address);
            // break the loop
            break
        }

        // trans bytes to String
        match String::from_utf8(Vec::from(&buffer[..read])){
            // success, print out the string message
            Ok(message) => println!("{} says {}", address, message),
            // fail, print out the original bytes message
            _ => println!("{} parse message failed. original message is {:?}", address, &buffer[..read])
        }
        // return message(echo)
        stream.write(&buffer[..read])?;
    }
    // print connection close info
    println!("{} shut down", address);
    // return
    Ok(())
}

fn main(){
    // create listener
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // print out the address of listening
    println!("Listening on {}", listener.local_addr().unwrap());
    // read connection
    for stream in listener.incoming(){
        // check the connection
        let stream = stream.unwrap();
        // create thread
        spawn(move || {
            // use handle_client to deal with this client
            handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err))
        });
    }
}
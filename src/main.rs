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
    // 创建buffer
    let mut buffer = [0; 512];
    // 保存连接地址
    let address = stream.peer_addr()?;
    // 输出连接地址
    println!("the address is {}", address);

    loop{
        // 读取输入流
        let read = stream.read(&mut buffer)?;
        // 检查信息
        if &buffer[..read] == [255, 244, 255, 253, 6]{
            // 输出退出信息
            println!("{} received exit signal", address);
            // break
            break
        }

        // 将字节转换为字符串
        match String::from_utf8(Vec::from(&buffer[..read])){
            // success, 输出字符信息
            Ok(message) => println!("{} says {}", address, message),
            // fail, 打印出原始字节消息
            _ => println!("{} parse message failed. original message is {:?}", address, &buffer[..read])
        }
        // return message(echo)
        stream.write(&buffer[..read])?;
    }
    // 打印关闭地址
    println!("{} shut down", address);
    // return
    Ok(())
}

fn main(){
    // 创建监听·
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // 打印监听地址
    println!("Listening on {}", listener.local_addr().unwrap());
    // 读取连接信息
    for stream in listener.incoming(){
        // 检查连接
        let stream = stream.unwrap();
        // 创建线程
        spawn(move || {
            // 使用 handle_client 来处理客户端请求
            handle_client(stream).unwrap_or_else(|err| eprintln!("error--s{:?}", err))
        });
    }
}
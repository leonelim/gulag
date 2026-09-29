use std::{fs, io::{self, Read, Write}, net::{TcpListener, TcpStream}, println};
fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    match buffer.as_str() {
        "r\n" => recieve_file(),
        "s\n" => {
            io::stdout().write("ip addr: ".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
            let mut ip_addr = String::new();
            let mut file_path = String::new();
            io::stdin().read_line(&mut ip_addr).unwrap();
            ip_addr.truncate(ip_addr.len() - 1);
            io::stdout().write("file name: ".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut file_path).unwrap();
            file_path.truncate(file_path.len() - 1);
            send_file(&(ip_addr + ":1234"), &file_path);
        },
        _ => {}
    }
}
fn recieve_file() {
    let server_socket = TcpListener::bind("0.0.0.0:1234").unwrap();
    let socket = server_socket.accept();
    let mut input_stream = socket.unwrap().0;
    let mut buff = Vec::new();
    let mut buf = [0u8; 8];
    input_stream.read_exact(&mut buf).unwrap();
    let size= usize::from_be_bytes(buf);
    println!("{}", size);
    let mut vec: Vec<u8> = vec![0u8; size];
    input_stream.read_exact(&mut vec).unwrap();
    input_stream.read_to_end(&mut buff).unwrap();
    fs::write(String::from_utf8(vec).unwrap(), buff).unwrap();
}
fn send_file(addr: &str, file_path: &str) {
    let mut socket = TcpStream::connect(addr).unwrap();
    let buf = file_path.len().to_be_bytes();
    socket.write(&buf);
    socket.write(file_path.as_bytes());
    socket.write(&fs::read(file_path).unwrap()).unwrap();
}

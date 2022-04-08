use std::process::Command;
use std::env;
use std::thread;

fn main() {
    println!("running {}...", env::args().skip(1).next().expect("no args"));
    loop {
        let mut args = env::args().skip(1);
        let mut command = Command::new(args.next().expect("no args"));

        

        for arg in args {
            command.arg(arg);
        }
        if command.spawn().expect("didn't launch").wait().expect("didn't end").success() {
            println!("Success!");
            break; 
        }
        else {
            println!("failed");

            thread::sleep_ms(800);
            println!("retrying");
        }
    }
}

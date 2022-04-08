use std::process::Command;
use std::env;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    println!("running {}...", env::args().skip(1).next().expect("no args"));

    let start = Instant::now();
    let mut tries = 0;

    loop {
        tries = tries + 1;
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
            thread::sleep(Duration::from_millis(800));
            println!("retrying");
        }
    }
    println!("took {} seconds, {} tries",start.elapsed().as_secs(), tries);
}

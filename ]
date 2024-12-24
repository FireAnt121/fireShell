use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        println!("> ");
        let _ = stdout().flush();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();

        // this is the main command to start
        let command = parts.next().unwrap();

        // these are the arguments passed to the main command
        let args = parts;

        // inbuilt commands
        match command {
            "cd" => {
                let dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(dir);

                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            _ => {
                let child = Command::new(command).args(args).spawn();
                match child {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }
}

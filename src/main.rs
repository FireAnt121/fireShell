use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::{Child, Command, Stdio},
    str::from_utf8,
};

fn main() {
    let mut current_git_branch = String::new();
    let git_check = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("true");

    let _ = stdout().flush();
    if git_check.success() {
        println!("its a git repo");
        let git_branch = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .unwrap();

        current_git_branch.push_str(match from_utf8(&git_branch.stdout) {
            Ok(v) => v,
            Err(_) => panic!("not a utf8"),
        });
    }

    loop {
        print!("# \x1b[93m{} \x1b[0m>", current_git_branch.trim());
        let _ = stdout().flush();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        // implementing for pipe
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
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
                    previous_command = None;
                }
                "exit" => return,
                _ => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            let _ = final_command.wait();
        };
    }
}

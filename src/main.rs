use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        // Print the shell prompt
        print!(" ~rsh~");
        print!(" > > > ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Remove the trailing newline character
        let input = input.trim();

        // Split the input into command and arguments
        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        // Handle built-in commands, such as `cd`
        if command == "cd" {
            if args.len() != 1 {
                eprintln!("Usage: cd <directory>");
            } else {
                let new_dir = args[0];
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("cd: {}: {}", new_dir, e);
                }
            }
            continue;
        }

        // Exit the shell
        if command == "exit" {
            break;
        }

        // Execute external commands
        match Command::new(command)
            .args(&args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
        {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command exited with non-zero status");
                }
            }
            Err(e) => eprintln!("Failed to execute command: {}", e),
        }
    }
}


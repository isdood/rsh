use std::env;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::os::unix::io::AsRawFd;
use termios::*;

fn main() {
    let stdin = 0; // stdin file descriptor
    let mut termios = Termios::from_fd(stdin).unwrap();
    let original_termios = termios.clone();

    // Set terminal to raw mode
    termios.c_lflag &= !(ICANON | ECHO); // Disable canonical mode and echo
    termios.c_cc[VMIN] = 1; // Minimum number of characters for noncanonical read
    termios.c_cc[VTIME] = 0; // Timeout in deciseconds for noncanonical read
    tcsetattr(stdin, TCSANOW, &termios).unwrap();

    let mut stdout = io::stdout();

    loop {
        // Print the shell prompt
        print!(" ~rsh~");
        print!(" > > > ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        loop {
            let mut buffer = [0; 1];
            let _ = io::stdin().read(&mut buffer).unwrap();
            match buffer[0] {
                b'\n' => break,
                b'\x1B' => {
                    // Handle escape sequences
                    let mut buffer = [0; 2];
                    io::stdin().read_exact(&mut buffer).unwrap();
                    match buffer {
                        [b'[', b'A'] => {
                            // Up arrow key
                        }
                        [b'[', b'B'] => {
                            // Down arrow key
                        }
                        [b'[', b'C'] => {
                            // Right arrow key
                        }
                        [b'[', b'D'] => {
                            // Left arrow key
                        }
                        _ => {}
                    }
                }
                b'\x7F' => {
                    // Handle backspace
                    if !input.is_empty() {
                        input.pop();
                        print!("\x08 \x08");
                        io::stdout().flush().unwrap();
                    }
                }
                c => {
                    input.push(c as char);
                    print!("{}", c as char);
                    io::stdout().flush().unwrap();
                }
            }
        }

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

    // Restore original terminal settings
    tcsetattr(stdin, TCSANOW, &original_termios).unwrap();
}

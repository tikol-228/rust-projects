use std::{env, fs::{self, File}, io, process::exit};

fn main() -> io::Result<()> {
    loop {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    println!("{:?}", parts);

    match parts.first() {
        Some(&"mkdir") => {
            if let Some(path) = parts.get(1) {
                fs::create_dir(path)?;
            }
        }
        Some(&"mkfile") => {
            if let Some(path) = parts.get(1) {
                File::create(path)?;
            }
        }
        Some(&"ls") => {
            if let Some(path) = parts.get(1) {
                for entry in fs::read_dir(path)? {
                println!("{}", entry?.path().display());
            }
            }
        }
        Some(&"cd") => {
            if let Some(path) = parts.get(1) {
                env::set_current_dir(path)?;
            }
        }
        Some(&"rmf") => {
            if let Some(path) = parts.get(1) {
                fs::remove_file(path)?;
            }
        }
        Some(&"rmdir") => {
            if let Some(path) = parts.get(1) {
                fs::remove_dir(path)?;
            }
        }
        Some(&"pwd") => {
            let pwd = env::current_dir()?;
            println!("Current directory is {}", pwd.display());
        }
        Some(&"h") => {
            println!("
                Available commands:

                mkdir  - create directory
                mkfile - create file
                ls     - list files
                cd     - change directory
                rmf    - remove file
                rmdir  - remove directory
                exit   - close program
            ")
        }
        Some(&"exit") => {
            print!("exiting from app");
            exit(0);
        }
        _ => println!("incorrect input")
    }
    }
    Ok(())
}

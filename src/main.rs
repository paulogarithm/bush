use std::env;
use std::fs::{self, Metadata};
use std::io::{self, Result, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{self, Command};

fn path_which(command: &mut String) -> Result<i8> {
    // Check for file existence
    let mut paths: Vec<String> = vec![];
    env::var_os("PATH").map(|term| paths.push(String::from(term.to_string_lossy())));

    // Check for file existence
    let mut realcommand: String = String::new();
    let metadata: Option<Metadata> = paths.iter().find_map(|path| {
        let potential_path = format!("{}/{}", path, command);
        if fs::metadata(&potential_path).is_ok() {
            realcommand = String::from(&potential_path);
        }
        return fs::metadata(&potential_path).ok();
    });

    // Change command and get metatada
    command.clear();
    command.push_str(&realcommand);
    if let Some(md) = metadata {

        // Check if input is a directory
        if md.is_dir() {
            eprintln!("{} is a directory.", command);
            return Ok(-1);
        }

        // Check for execute permissions
        let perm: fs::Permissions = md.permissions();
        if perm.mode() & 0b111 == 0 {
            eprintln!("Permission denied.");
            return Ok(-1);
        }
    }
    return Err(io::Error::from_raw_os_error(0));
}

fn treat_input(str: String) -> Result<i8> {
    // Split the words
    let mut words: Vec<&str> = str.split(' ').collect();
    if words.is_empty() || str.is_empty() {
        return Ok(0);
    }

    // Get the command
    let mut command: &&str = &("");
    let clone: Vec<&str> = words.clone();
    if let Some(command_getter) = clone.first().clone() {
        command = command_getter;
    }
    words.remove(0);

    // Check for file
    match path_which(&mut command.to_string()) {
        Ok(n) => return Ok(n),
        Err(_) => (),
    }

    // Execute the command
    return Ok(Command::new(command)
        .args(&words)
        .spawn()
        .map(|mut child| child.wait().expect("Failed to wait for child process"))
        .map(|status| status.code().map(|code| code as i8).unwrap_or(-1))
        .map_err(|err| eprintln!("Failed to execute command: {}", err))
        .unwrap_or(-1));
}

fn support_colors() -> bool {
    return env::var_os("TERM")
        .map(|term| term.to_string_lossy().to_lowercase().contains("color"))
        .unwrap_or(false);
}

fn main() -> Result<()> {
    let mut code = 0;
    let color_supported = support_colors();

    loop {
        let mut buffer: String = String::new();
        print!("[");
        if color_supported {
            print!("{}", if code == 0 { "\x1b[32m" } else { "\x1b[31m" })
        }
        print!(
            "{}{}] > ",
            code,
            if color_supported { "\x1b[m" } else { "" },
        );
        io::stdout().flush()?;
        let result: Result<usize> = io::stdin().read_line(&mut buffer);

        match result {
            Ok(0) => {
                println!("exit");
                break;
            }
            Ok(_) => {
                buffer.pop();
                match treat_input(buffer) {
                    Ok(n) => code = n,
                    Err(err) => return Err(err),
                }
            }
            Err(err) => eprintln!("Readline: {}", err),
        }
    }
    process::exit(code as i32);
}

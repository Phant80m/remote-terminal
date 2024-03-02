#[macro_export]
macro_rules! error {
    ($message:expr) => {
        eprintln!(
            "{} {}",
            "[  ]".red().bold(),
            $message
        );
    };
    ($message:expr, $($arg:expr),*) => {
        eprintln!(
            "{} {}",
            "[  ]".red().bold(),
            format!($message, $($arg),*)
        );
    };
}

// pub fn cmd(args: Vec<String>) -> String {
//     // Execute the command
//     let output: Output = Command::new(&args[0])
//         .args(&args[1..])
//         .output()
//         .expect("Failed to execute command");

//     // Convert the output bytes to a string
//     String::from_utf8_lossy(&output.stdout).to_string()
// }

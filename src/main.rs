use async_std::println;
use clap::Parser;
use http_types::headers::HeaderValue;
use std::process::{Command, Output};
// use remote_exec::cmd;
use tide::prelude::*;
use tide::security::{CorsMiddleware, Origin};
use tide::Request;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    adress: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Executor {
    command: String,
    args: Vec<String>,
}
pub fn cmd(args: Vec<String>) -> String {
    // Check if the command is 'cd'
    if args[0] == "cd" {
        if let Err(err) = std::env::set_current_dir(&args[1]) {
            return format!("Failed to change directory: {}", err);
        }
        return String::new();
    }

    let output: Output = Command::new(&args[0])
        .args(&args[1..])
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args = Args::parse();
    let addr = args.adress.unwrap_or("127.0.0.1:8000".to_string());
    let mut app = tide::new();
    app.with(
        CorsMiddleware::new()
            .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from("*"))
            .allow_credentials(false),
    );
    app.at("/exec").post(exec);
    app.at("/cwd").get(current_dir);
    println!("server listening at: {}", &addr).await;
    app.listen(&addr).await?;
    Ok(())
}
async fn current_dir(_: Request<()>) -> tide::Result {
    let current_directory = std::env::current_dir().expect("current directory");

    Ok(json!({
        "current_directory": current_directory.to_string_lossy()
    })
    .into())
}
async fn exec(mut req: Request<()>) -> tide::Result {
    let Executor { command, mut args } = req.body_json().await?;
    args.insert(0, command.clone());
    let output = cmd(args.clone());
    Ok(json!({
        "command_entered": format!("$: {}", &args.join(" ")),
        "output": output
    })
    .into())
}

// use std::io::{BufRead, BufReader};
use env_logger::{Builder, Target};
use log::*;
use std::io::Write;
use subprocess::{Popen, PopenConfig, Redirection};

#[cfg(target_os = "windows")]
static CLIENT_PATH: &str = "..\\process_client\\target\\debug\\process_client.exe";

#[cfg(not(target_os = "windows"))]
static CLIENT_PATH: &str = "../process_client/target/debug/process_client";
fn main() -> Result<(), std::io::Error> {
    let mut builder = Builder::new();
    builder
        .format(|buf, record| writeln!(buf, "SERVER {} - {}", record.level(), record.args()))
        .target(Target::Stderr)
        .filter(None, LevelFilter::Debug)
        .init();

    debug!("Hello, world!");

    let mut p = Popen::create(
        &[CLIENT_PATH, "arg1", "arg2"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::Pipe,
            ..Default::default()
        },
    )
    .unwrap();
    let commands = vec!["test", "exit"];
    for command in commands {
        debug!("sending command: {}", command);
        let mut communicator = p.communicate_start(Some(command.as_bytes().to_vec()));
        debug!("got communicator");
        if let Ok((out, err)) = communicator.read() {
            debug!("called communicate");
            if let Some(out) = out {
                debug!("SERVER out: {}", String::from_utf8_lossy(&out));
            }
            if let Some(err) = err {
                debug!("SERVER err: {}", String::from_utf8_lossy(&err));
            }
        }
        if let Some(exit_status) = p.poll() {
            debug!("child process exited with: {:?}", exit_status);
            break;
        }
    }
    p.terminate()
}

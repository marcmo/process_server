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
    loop {
        let (out, err) = p.communicate(Some("sending test"))?;
        if let Some(out) = out {
            debug!("SERVER out: {}", &out);
        }
        if let Some(err) = err {
            debug!("SERVER err: {}", &err);
        }
        // check if the process is still alive
        if let Some(exit_status) = p.poll() {
            debug!("child process exited with: {:?}", exit_status);
        } else {
            // it is still running, terminate it
            p.terminate()?;
        }
    }
}

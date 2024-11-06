use std::error;
use std::io::Error;
use std::process::Stdio;
use std::time::Duration;
use log::{error, trace};
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::process::{Child, Command};
use tokio::time::timeout;
use crate::config::Config;

pub struct Chromedriver {
    child: Child,
    pub port: u16,
}

impl Chromedriver {
    pub async fn launch(env: &Config) -> Result<Chromedriver, Box<dyn error::Error>> {
        let port = find_free_port().await?;
        let mut child = Command::new(env.chromedriver.clone()
            .unwrap_or("chromedriver".to_string())
        )
            .process_group(0)
            .arg(format!("--port={port}"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        match timeout(Duration::from_millis(500), child.wait()).await {
            Ok(status) => {
                if let Some(status_code) = status.expect("child.wait() errored").code() {
                    error!("chromedriver exited with code {}", status_code);
                }
                let mut stderr = String::new();
                child.stderr
                    .expect("chromedriver did fail but not output to stderr")
                    .read_to_string(&mut stderr)
                    .await?;
                Err(Box::new(Error::other(format!("chromedriver failed to start: {}", stderr))))
            }
            Err(_) => {
                // This branch means the timeout occurred, so the chromedriver did not exit –  this
                // is the positive case.
                let reader = BufReader::new(
                    child.stdout.take().expect("Stdout should be there")
                );
                let stdout = reader.lines().next_line().await?
                    .expect("chromedriver should have output something");
                trace!("chromedriver started: \"{}\"", stdout);
                Ok(Chromedriver { port, child })
            }
        }
    }
    pub async fn quit(&mut self) -> io::Result<()> {
        self.child.kill().await
    }
}

async fn find_free_port() -> io::Result<u16> {
    let listener = TcpListener::bind(("localhost", 0)).await?;
    listener.local_addr().map(|it| it.port())
}
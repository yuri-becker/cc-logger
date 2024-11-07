mod chromedriver;
mod config;
mod runner;

use crate::chromedriver::Chromedriver;
use crate::config::Config;
use crate::runner::{Command, Runner};
use dotenv::dotenv;
use log::{debug, error, trace};
use std::env::args;
use std::error::Error;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let result = run().await;
            if let Err(e) = result {
                error!("{}", e);
            }
        })
}

async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    dotenv::from_filename(".env.local").ok();
    pretty_env_logger::init();
    let args: Vec<String> = args().collect();

    let command = match args.get(1).map(|s| s.as_str()) {
        Some("come") => Command::Come,
        Some("leave") => Command::Leave,
        Some("info") => Command::Info,
        _ => {
            println!(
                "Usage:\t{} come\t– Logs \"Kommen\"\
                    \n\t{} leave\t– Logs \"Gehen\"\
                    \n\t{} info\t– Shows \"Ihre letzte Buchung\"",
                args[0], args[0], args[0]);
            return Err("Invalid command, exiting.".into());
        }
    };

    let config = Config::from_dotenv();
    let mut chromedriver = Chromedriver::launch(&config).await?;
    trace!("chromedriver started on port {}", chromedriver.port);
    debug!("Using {:?}", config);
    if let Err(e) = Runner::init(&chromedriver.port, &config).await?.run(command).await {
        error!("Error running the automation: {}", e);
    }
    if let Err(e) = chromedriver.quit().await {
        error!("Could not quit chromedriver: {}", e);
    } else {
        trace!("Quit chromedriver");
    }
    Ok(())
}

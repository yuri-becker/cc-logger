use crate::config::Config;
use fantoccini::{Client, ClientBuilder, Locator};
use log::{debug, trace};
use std::error::Error;
use std::time::Duration;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use fantoccini::wd::Capabilities;
use rand::Rng;
use serde_json::json;
use tokio::time::sleep;

pub enum Command { Come, Leave }

pub struct Runner<'r> {
    client: Client,
    config: &'r Config,
}

impl<'r> Runner<'r> {
    pub async fn init(port: &'r u16, config: &'r Config) -> Result<Self, Box<dyn Error>> {
        let connect = format!("http://localhost:{}", port);
        trace!("Connecting to {}", connect);
        let mut caps = Capabilities::new();
        caps.insert(
            "goog:chromeOptions".to_string(),
            json!({"args": vec![
                if config.no_sandbox.unwrap_or(false) {"--no-sandbox"} else {""},
                if config.headless.unwrap_or(true) {"--headless"} else {""}
            ].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>() }),
        );
        let client = ClientBuilder::native()
            .capabilities(caps)
            .connect(&connect).await?;

        if let Some(range_minutes) = config.random_sleep_range_minutes {
            let range_seconds: u16 = (range_minutes as u16) * 60;
            let sleep_seconds = rand::thread_rng().gen_range(0..=range_seconds);
            debug!("Sleeping for {} seconds.", sleep_seconds);
            sleep(Duration::from_secs(sleep_seconds as u64)).await;
        }

        client.goto(&config.url).await?;
        Ok(Runner { client, config })
    }

    pub async fn run(self, command: Command) -> Result<(), Box<dyn Error>> {
        self.ensured_logged_in().await?;
        self.open_timetracking().await?;
        match command {
            Command::Come => self.ensure_came().await?,
            Command::Leave => self.ensure_left().await?,
        }
        self.client.close().await?;
        Ok(())
    }

    async fn ensured_logged_in(&self) -> Result<(), CmdError> {
        let login = self.get_nav_link("Login").await;
        if login.is_err() {
            debug!("Could not find login link, therefore user is logged in.");
            return Ok(());
        }
        login?.click().await.expect("Could not click login link.");
        self.client.find(Locator::Id("login_username")).await?
            .send_keys(&self.config.username).await?;
        self.client.find(Locator::Id("login_passwort")).await?
            .send_keys(&self.config.password).await?;
        self.client.find(Locator::Css("input[type='submit'][name='login_submit']")).await?
            .click().await?;
        debug!("Logged in.");
        Ok(())
    }

    async fn open_timetracking(&self) -> Result<(), CmdError> {
        self.get_nav_link("- Zeiterfassung").await?.click().await?;
        self.client.find(Locator::Id("zeiterfassungdetailscontainer")).await?
            .find(Locator::Css("button")).await?
            .click().await?;
        sleep(Duration::from_secs(3)).await; // The dialog opens very slowly
        Ok(())
    }

    async fn ensure_left(&self) -> Result<(), CmdError> {
        let gehen = self.client.find(Locator::Id("kugDialog")).await?
            .find(Locator::Css("input.buttonGehen")).await;
        if gehen.is_err() {
            debug!("Could not find Leave button, therefore already left.");
            return Ok(());
        }
        debug!("Successfully logged left");
        gehen?.click().await
    }

    async fn ensure_came(&self) -> Result<(), CmdError> {
        let kommen = self.client.find(Locator::Id("kugDialog")).await?
            .find(Locator::Css("input.buttonKommen")).await;
        if kommen.is_err() {
            debug!("Could not find 'Kommen', therefore already went");
            return Ok(());
        }
        kommen?.click().await
    }

    async fn get_nav_link(&self, link_text: &str) -> Result<Element, CmdError> {
        let nav = self.client.find(Locator::Id("nav")).await?;
        nav.find(Locator::Css(".left-navi")).await?
            .find(Locator::LinkText(link_text)).await
    }
}
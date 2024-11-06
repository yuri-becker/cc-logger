use dotenv::var;
use std::fmt::Formatter;
use std::ops::Deref;


pub struct Password {
    inner: String,
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("\"***\"").finish()
    }
}

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<String> for Password {
    fn from(s: String) -> Self {
        Password { inner: s }
    }
}
#[derive(Debug)]
pub struct Config {
    pub chromedriver: Option<String>,
    pub url: String,
    pub username: String,
    pub password: Password,
    pub random_sleep_range_minutes: Option<u8>,
    pub no_sandbox: Option<bool>,
    pub headless: Option<bool>,
}

impl Config {
    pub fn from_dotenv() -> Self {
        Config {
            chromedriver: var("COMCAVE_CHROMEDRIVER").ok(),
            url: var("COMCAVE_URL").expect("COMCAVE_URL must be set"),
            username: var("COMCAVE_USERNAME").expect("COMCAVE_USERNAME must be set"),
            password: var("COMCAVE_PASSWORD").expect("COMCAVE_PASSWORD must be set").into(),
            random_sleep_range_minutes: var("COMCAVE_RANDOM_SLEEP_RANGE")
                .ok()
                .map(|s| s.parse::<u8>()
                    .expect("COMCAVE_RANDOM_SLEEP_RANGE must be lower than 256")
                ),
            no_sandbox: var("COMCAVE_NO_SANDBOX")
                .ok()
                .map(|s| s.parse::<bool>()
                    .expect("COMCAVE_NO_SANDBOX must be true or false")
                ),
            headless: var("COMCAVE_HEADLESS")
                .ok()
                .map(|s| s.parse::<bool>()
                    .expect("COMCAVE_HEADLESS must be true or false")
                ),
        }
    }
}
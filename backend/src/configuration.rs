#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    prod_environment: bool,
    secret_key: String,

    dev_app_host: String,
    dev_db_username: String,
    dev_db_password: String,
    dev_db_host: String,
    dev_db_name: String,
    // dev_logger_level: String,
    sandbox_paypal_api_client_id: String,
    sandbox_paypal_api_secret: String,
    sandbox_paypal_base_url: String,

    prod_app_host: String,
    prod_db_username: String,
    prod_db_host: String,
    prod_db_password: String,
    prod_db_name: String,
    // prod_logger_level: String
    live_paypal_api_client_id: String,
    live_paypal_api_secret: String,
    live_paypal_base_url: String,
}

pub fn get_configuration() -> Settings {
    let file = std::fs::File::open("config.yaml").expect("Couldn't open config.yaml");
    let scrape_config: Settings = serde_yaml::from_reader(file).unwrap();

    scrape_config
}

impl Settings {
    pub fn print_environtment(&self) {
        println!("Is production environtment?: {}", self.prod_environment);
    }

    pub fn is_prod_environment(&self) -> bool {
        self.prod_environment
    }

    pub fn db_connection_string(&self) -> String {
        match self.prod_environment {
            true => format!(
                "postgres://{}:{}@{}/{}",
                self.prod_db_username, self.prod_db_password, self.prod_db_host, self.prod_db_name
            ),
            false => format!(
                "postgres://{}:{}@{}/{}",
                self.dev_db_username, self.dev_db_password, self.dev_db_host, self.dev_db_name
            ),
        }
    }

    pub fn app_local_address(&self) -> String {
        match self.prod_environment {
            true => format!("{}:{}", self.prod_app_host, self.application_port),
            false => format!("{}:{}", self.dev_app_host, self.application_port),
        }
    }

    pub fn get_secret_key(&self) -> String {
        format!("{}", self.secret_key)
    }

    pub fn get_paypal_credentials(&self) -> (String, String, String) {
        match self.prod_environment {
            true => (
                format!("{}", self.live_paypal_api_client_id),
                format!("{}", self.live_paypal_api_secret),
                format!("{}", self.live_paypal_base_url),
            ),
            false => (
                format!("{}", self.sandbox_paypal_api_client_id),
                format!("{}", self.sandbox_paypal_api_secret),
                format!("{}", self.sandbox_paypal_base_url),
            ),
        }
    }
}

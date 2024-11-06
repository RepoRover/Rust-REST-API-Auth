pub const ENV_FILE_PATH: &str = "./environment/.env";

pub const ENV_VAR_PREFIX: &str = "RRAA_";

// In order for application to work properly make sure these
// variables are bound to REQUIRED_ENV_VARS array!
pub const DB_PROTOCOL: &str = "DB_PROTOCOL";
pub const DB_USER: &str = "DB_USER";
pub const DB_PASSWORD: &str = "DB_PASSWORD";
pub const DB_HOST: &str = "DB_HOST";
pub const DB_PORT: &str = "DB_PORT";
pub const DB_NAME: &str = "DB_NAME";
pub const DB_SLL_MODE: &str = "DB_SLL_MODE";
pub const DB_SSL_CERTIFICATE_PATH: &str = "DB_SSL_CERTIFICATE_PATH";
pub const APP_ADDRESS: &str = "APP_ADDRESS";
pub const APP_PORT: &str = "APP_PORT";

// Update this array with each new required environment variable!
pub const REQUIRED_ENV_VARS: [&str; 10] = [
    DB_PROTOCOL,
    DB_USER,
    DB_PASSWORD,
    DB_HOST,
    DB_PORT,
    DB_NAME,
    DB_SLL_MODE,
    DB_SSL_CERTIFICATE_PATH,
    APP_ADDRESS,
    APP_PORT,
];

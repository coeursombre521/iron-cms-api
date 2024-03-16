use std::env::{self, VarError};

use ansi_term::Color;
use dotenv::dotenv;
use tracing::{info, warn, error};

use crate::constants as main_constants;
use crate::domain::constants as domain_constants;
use crate::services::constants as services_constants;

const ENV_CHECK_MSG_FAILED_TO_START: &str = "Program failed to start.";
const ENV_CHECK_MSG_VARIABLE_MISSING: &str = "Variable is missing.";
const ENV_CHECK_MSG_VARIABLE_UNICODE: &str = "Variable might be of invalid format.";
const ENV_CHECK_MSG_DEFAULT_FALLBACK: &str = "Using default value when needed.";
const ENV_CHECK_MSG_REQUIRED_VARS_PRESENT: &str = "All required variables are present.";
const ENV_CHECK_MSG_EVERYTHING_OK: &str = "Everything seems okay. Program is ready to start.";

pub fn check_env_variables() {
    dotenv().ok();

    let required_vars = vec![
        main_constants::ENV_SERVER_DOMAIN,
        main_constants::ENV_SERVER_PORT,
        domain_constants::POSTGRESQL_DB_URI 
    ];
    let recommended_vars = vec![
        services_constants::SEC_ARGON2ID_ENV_MEMORY_SIZE_MB,
        services_constants::SEC_ARGON2ID_ENV_NUM_ITERATIONS,
        services_constants::SEC_ARGON2ID_ENV_NUM_THREADS,
        services_constants::SEC_ARGON2ID_ENV_OUTPUT_LEN
    ];
    let mut is_everything_ok = true;

    for var in required_vars {
        match env::var(var) {
            Ok(_) => {},
            Err(VarError::NotPresent) => {
                error!("{} - {} {}", Color::Red.bold().paint(var), ENV_CHECK_MSG_FAILED_TO_START, ENV_CHECK_MSG_VARIABLE_MISSING);
                is_everything_ok = false;
            }
            Err(VarError::NotUnicode(_)) => {
                error!("{} - {} {}", Color::Red.bold().paint(var), ENV_CHECK_MSG_FAILED_TO_START, ENV_CHECK_MSG_VARIABLE_UNICODE);
                is_everything_ok = false;
            }
        }
    }

    if !is_everything_ok {
        std::process::exit(1);
    }

    info!("{}", Color::Green.bold().paint(ENV_CHECK_MSG_REQUIRED_VARS_PRESENT));

    for var in recommended_vars {
        match env::var(var) {
            Ok(_) => {},
            Err(VarError::NotPresent) => {
                warn!("{} - {} {}", Color::Yellow.bold().paint(var), ENV_CHECK_MSG_VARIABLE_MISSING, ENV_CHECK_MSG_DEFAULT_FALLBACK);
            }
            Err(VarError::NotUnicode(_)) => { 
                warn!("{} - {} {}", Color::Yellow.bold().paint(var), ENV_CHECK_MSG_VARIABLE_UNICODE, ENV_CHECK_MSG_DEFAULT_FALLBACK);
            }
        }
    }

    info!("{}", Color::Green.bold().paint(ENV_CHECK_MSG_EVERYTHING_OK));
}
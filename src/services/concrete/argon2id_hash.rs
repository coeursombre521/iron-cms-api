use std::time::{Instant, Duration};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2,
    Algorithm,
    Params,
    Version
};
use tracing::info;

use crate::services::{
    error::SecurityError, 
    utils::{
        envutil::get_env_var_as_str,
        envutil::get_env_var_as_type_or_default,
        format::format_error_string
    }
};
use crate::services::traits::password_hash::PasswordHashService;
use crate::services::utils::format;
use crate::services::constants;

pub struct Argon2IdHashService<'a> {
    argon2: Argon2<'a>,
    time_start: Instant
}

impl<'a> Argon2IdHashService<'a> {
    pub fn new() -> Self {
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(
                Self::retrieve_memory_size_from_env(),
                Self::retrieve_num_iterations_from_env(),
                Self::retrieve_num_threads_from_env(),
                Some(Self::retrieve_output_len_from_env())
            ).unwrap()
        );

        Self {
            argon2,
            time_start: Instant::now()
        }
    }

    fn retrieve_memory_size_from_env() -> u32 {
        get_env_var_as_type_or_default(constants::SEC_ARGON2ID_ENV_MEMORY_SIZE_MB, &constants::SEC_ARGON2ID_MEMORY_SIZE_MB_DEFAULT)
    }

    fn retrieve_num_iterations_from_env() -> u32 {
        get_env_var_as_type_or_default(constants::SEC_ARGON2ID_ENV_NUM_ITERATIONS, &constants::SEC_ARGON2ID_NUM_ITERATIONS_DEFAULT)
    }

    fn retrieve_num_threads_from_env() -> u32 {
        get_env_var_as_type_or_default(constants::SEC_ARGON2ID_ENV_NUM_THREADS, &constants::SEC_ARGON2ID_NUM_THREADS_DEFAULT)
    }

    fn retrieve_output_len_from_env() -> usize {
        get_env_var_as_type_or_default(constants::SEC_ARGON2ID_ENV_OUTPUT_LEN, &constants::SEC_ARGON2ID_OUTPUT_LEN_DEFAULT)
    }

    fn is_log_debug_or_trace() -> bool {
        let debug = get_env_var_as_str("RUST_LOG");
        debug.is_ok() && (debug.as_ref().unwrap() == "debug" || debug.as_ref().unwrap() == "trace")
    }
}

impl<'a> Default for Argon2IdHashService<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PasswordHashService for Argon2IdHashService<'a> {
    fn hash_password(&self, password: &str) -> Result<String, SecurityError> {
        let salt = SaltString::generate(&mut OsRng);

        if Self::is_log_debug_or_trace() {
            info!("Generated salt: {}", salt.as_str());
        }

        let mut duration_1: Option<Duration> = None;
        if Self::is_log_debug_or_trace() {
            duration_1 = Some(self.time_start.elapsed());
        }

        let password_hash = self.argon2.hash_password(password.as_bytes(), &salt);

        if Self::is_log_debug_or_trace() {
            let duration_2 = self.time_start.elapsed();
            let elapsed = duration_2 - duration_1.unwrap();
            info!("Hashing took {} ms", elapsed.as_millis());
        }

        match password_hash {
            Ok(hash) => Ok(hash.to_string()),
            Err(err) => Err(SecurityError {
                message: format::format_error_string(constants::SEC_ERR_HASH_FAILED, err.to_string().as_str()),
                context: constants::ERR_CONTEXT_ARGON2ID_SERV.to_string(),
            })
        }
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, SecurityError> {
        let password_hash = match PasswordHash::new(hash) {
            Ok(hash) => hash,
            Err(err) => return Err(SecurityError {
                message: format::format_error_string(constants::SEC_ERR_HASH_PARSE_FAIL, err.to_string().as_str()),
                context: constants::ERR_CONTEXT_ARGON2ID_SERV.to_string(),
            })
        };
        
        let mut duration_1: Option<Duration> = None;
        if Self::is_log_debug_or_trace() {
            duration_1 = Some(self.time_start.elapsed());
        }

        match self.argon2.verify_password(password.as_bytes(), &password_hash) {
            Ok(_) => {
                if Self::is_log_debug_or_trace() {
                    let duration_2 = self.time_start.elapsed();
                    let elapsed = duration_2 - duration_1.unwrap();
                    info!("Verifying took {} ms", elapsed.as_millis());
                }
                Ok(true)
            },
            Err(err) => {
                if Self::is_log_debug_or_trace() {
                    let duration_2 = self.time_start.elapsed();
                    let elapsed = duration_2 - duration_1.unwrap();
                    info!("Verifying took {} ms", elapsed.as_millis());
                }
                Err(SecurityError {
                    message: format_error_string(constants::SEC_ERR_PASS_VERIFY, err.to_string().as_str()),
                    context: constants::ERR_CONTEXT_ARGON2ID_SERV.to_string(),
                })
            }
        }
    }
}
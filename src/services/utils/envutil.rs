use std::env;
use std::str;

use crate::services::error::EnvVariableError;
use crate::services::constants::ENV_ERR_PARSE_FAIL;

pub fn get_env_var_as_str(var_name: &str) -> Result<String, EnvVariableError> {
    match env::var(var_name) {
        Ok(val) => Ok(val),
        Err(err) => Err(EnvVariableError {
            message: err.to_string(),
            variable_name: var_name.to_string(),
        }),
    }
}

pub fn get_env_var_as_type<T: std::str::FromStr + Default>(var_name: &str) -> Result<T, EnvVariableError> {
    match get_env_var_as_str(var_name) {
        Ok(val) => match val.parse::<T>() {
            Ok(val) => Ok(val),
            Err(_) => Err(EnvVariableError {
                message: ENV_ERR_PARSE_FAIL.to_string(),
                variable_name: var_name.to_string(),
            }),
        },
        Err(err) => Err(err),
    }
}

pub fn get_env_var_as_type_or_default<T: str::FromStr + Default + Copy>(var_name: &str, default_value: &T) -> T {
    match get_env_var_as_type(var_name) {
        Ok(val) => val,
        Err(_) => *default_value
    }
}
pub mod constants;

use constants::*;
use std::collections::HashSet;
use std::env;

use crate::controller::{AppError, ErrorKind, Result};

pub fn get_var_name(constant: &str) -> String {
    format!("{}{}", ENV_VAR_PREFIX, constant)
}

pub fn load_app_env(path: &str) -> Result<()> {
    println!("Loading environment file contents...");

    load_env_file(path)?;

    let provided_vars: HashSet<String> = collect_provided_vars();

    check_required_vars(&provided_vars)?;
    print_extra_vars(&provided_vars);

    println!("All required environment variables are set.");

    Ok(())
}

fn load_env_file(path: &str) -> Result<()> {
    if let Err(err) = dotenvy::from_filename(path) {
        return Err(AppError::new(
            ErrorKind::InvalidEnvFile(format!(
                "Environment file is invalid at provided path: {}.",
                path
            )),
            Some(Box::new(err)),
        ));
    }

    Ok(())
}

fn collect_provided_vars() -> HashSet<String> {
    env::vars()
        .filter_map(|(key, _value)| {
            if key.starts_with(ENV_VAR_PREFIX) {
                Some(key[ENV_VAR_PREFIX.len()..].to_string()) // Strip prefix
            } else {
                None
            }
        })
        .collect()
}

fn check_required_vars(provided_vars: &HashSet<String>) -> Result<()> {
    let missing_vars: Vec<&str> = REQUIRED_ENV_VARS
        .iter()
        .filter(|&&req_var| !provided_vars.contains(req_var))
        .copied()
        .collect();

    if !missing_vars.is_empty() {
        return Err(AppError::new(
            ErrorKind::MissingEnvVars(format!(
                "Missing required environment variables: {}",
                missing_vars.join(", ")
            )),
            None,
        ));
    }

    Ok(())
}

fn print_extra_vars(provided_vars: &HashSet<String>) {
    let extra_vars: Vec<String> = provided_vars
        .iter()
        .filter(|var| !REQUIRED_ENV_VARS.contains(&var.as_str()))
        .cloned()
        .collect();

    if !extra_vars.is_empty() {
        println!(
            "Note: Extra environment variables with prefix '{}' found: {}",
            ENV_VAR_PREFIX,
            extra_vars.join(", ")
        );
    }
}

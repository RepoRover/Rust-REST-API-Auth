pub mod constants;

use constants::*;
use std::collections::HashSet;
use std::env;

use crate::controller::{AppError, ErrorKind, Result};

// TODO: Write integration test for this function

/// Loads the application environment from the specified file and checks for required variables.
///
/// # Arguments
///
/// * `path` - Path to the environment file.
///
/// # Returns
///
/// * `Result<(), AppError>` - An empty `Ok` result if successful, or an AppError if any required variables are missing,
///
/// # Errors
///
/// Returns an error if loading the environment file fails or if required environment variables are missing.
pub fn load_app_env(path: &str) -> Result<()> {
    println!("Loading environment file contents...");
    load_env_file(path)?;

    let provided_vars: HashSet<String> = collect_provided_vars(env::vars(), ENV_VAR_PREFIX);

    check_env(&REQUIRED_ENV_VARS, &provided_vars, ENV_VAR_PREFIX)?;
    print_extra_vars(&REQUIRED_ENV_VARS, &provided_vars, ENV_VAR_PREFIX)?;

    println!("All required environment variables are set.");
    Ok(())
}

// TODO: Write integration test for this function
// TODO: Write documentation for this function
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

// TODO: Write documentation for this function
fn collect_provided_vars<I>(env_iter: I, app_var_prefix: &str) -> HashSet<String>
where
    I: Iterator<Item = (String, String)>,
{
    env_iter
        .filter_map(|(key, _value)| {
            if key.starts_with(app_var_prefix) {
                Some(key.to_string())
            } else {
                None
            }
        })
        .collect()
}

// TODO: Write unit test
// TODO: Write documentation for this function
fn check_env(
    required_vars: &[&str],
    provided_vars: &HashSet<String>,
    app_var_prefix: &str,
) -> Result<()> {
    let missing_vars: Vec<String> = required_vars
        .iter()
        .filter_map(|&var| {
            let full_var_name = format!("{}{}", app_var_prefix, var);
            if !provided_vars.contains(&full_var_name) {
                Some(full_var_name)
            } else {
                None
            }
        })
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

// TODO: Write unit test
// TODO: Write documentation for this function
fn print_extra_vars(
    required_vars: &[&str],
    provided_vars: &HashSet<String>,
    app_var_prefix: &str,
) -> Result<()> {
    let extra_vars: Vec<String> = provided_vars
        .iter()
        .filter(|var| !required_vars.contains(&&var[app_var_prefix.len()..]))
        .cloned()
        .collect();

    if !extra_vars.is_empty() {
        let err_msg: String = format!("Detected variables with provided application prefix '{}' in current environment but they are not declared in 'REQUIRED_ENV_VARS'. Deteted variables: {}.", ENV_VAR_PREFIX, extra_vars.join(", "));
        return Err(AppError::new(ErrorKind::ExtraEnvVars(err_msg), None));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collecting_environment_variables() {
        let app_var_prefix: &str = "APP_PREFIX_";
        let var1: &str = "VAR_1";
        let var2: &str = "VAR_2";

        let constructed_var1: String = format!("{}{}", app_var_prefix, var1);
        let constructed_var2: String = format!("{}{}", app_var_prefix, var2);
        let orphan_var: String = "ORPHAN".to_string();

        let collected_vars = [
            (constructed_var1.clone(), "VALUE_1".to_string()),
            (constructed_var2.clone(), "VALUE_2".to_string()),
            (orphan_var, "VALUE_3".to_string()),
        ];

        let expected: HashSet<String> = HashSet::from([constructed_var1, constructed_var2]);

        let result: HashSet<String> =
            collect_provided_vars(collected_vars.iter().cloned(), &app_var_prefix);

        assert_eq!(expected, result);
    }
}

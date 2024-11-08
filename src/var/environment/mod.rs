pub mod constants;

use constants::*;
use std::collections::HashSet;
use std::env;

use crate::{
    controller::{AppError, ErrorKind, Result},
    find_missing,
};

// TODO: Write integration test for this functions
/// Loads the application environment from the specified file and checks for required variables.
///
/// # Arguments
///
/// * `path` - Path to the environment file.
///
/// # Returns
///
/// * `Result<()>` - An empty `Ok` result if successful.
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
/// Loads the application environment from the specified file.
///
/// # Arguments
///
/// * `path` - Path to the environment file.
///
/// # Returns
///
/// * `Result<(), AppError>` - An empty `Ok` result if successful.
///
/// # Errors
///
/// Returns AppError with InvalidEnvFile error kind followed by origianl error from dotenvy.
fn load_env_file(path: &str) -> Result<()> {
    dotenvy::from_filename(path).map_err(|err| {
        AppError::new(
            ErrorKind::InvalidEnvFile(format!(
                "Environment file is invalid at provided path: {}.",
                path
            )),
            Some(Box::new(err)),
        )
    })?;
    Ok(())
}

/// Filters environment variables intended to be used in the application from all the other environment variables by checking the application prefix on variables.
///
/// # Arguments
///
/// * `env_iter` - Iterator over all the environment variables.
/// * `app_var_prefix` - Prefix to filter the variables.
///
/// # Returns
///
/// * `HashSet<String>` - Unordered collection of variables that start with the provided prefix.
fn collect_provided_vars<I>(env_iter: I, app_var_prefix: &str) -> HashSet<String>
where
    I: Iterator<Item = (String, String)>,
{
    env_iter
        .filter_map(|(key, _value)| {
            if key.starts_with(app_var_prefix) {
                Some(key)
            } else {
                None
            }
        })
        .collect::<HashSet<String>>()
}

// TODO: Write unit test and docs
fn check_env(
    required_vars: &[&str],
    provided_vars: &HashSet<String>,
    app_var_prefix: &str,
) -> Result<()> {
    let full_var_names: HashSet<String> = required_vars
        .iter()
        .map(|&var| [app_var_prefix, var].concat())
        .collect();

    let missing_vars: HashSet<String> = find_missing(&full_var_names, provided_vars);

    if !missing_vars.is_empty() {
        return Err(AppError::new(
            ErrorKind::MissingEnvVars(format!(
                "Missing required environment variables: {}",
                missing_vars
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            None,
        ));
    }

    Ok(())
}

// TODO: Write unit test
// TODO: Write documentation for this function
fn is_required_var(var: &str, required_vars: &[&str], app_var_prefix: &str) -> bool {
    required_vars.contains(&&var[app_var_prefix.len()..])
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
        .filter(|var| !is_required_var(var, required_vars, app_var_prefix))
        .cloned()
        .collect();

    if !extra_vars.is_empty() {
        let err_msg = format!(
            "Extra variables not in REQUIRED_ENV_VARS detected: {}.",
            extra_vars.join(", ")
        );
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

        let all_vars: [(String, String); 3] = [
            (constructed_var1.clone(), "VALUE_1".to_string()),
            (constructed_var2.clone(), "VALUE_2".to_string()),
            (orphan_var, "VALUE_3".to_string()),
        ];

        let expected: HashSet<String> = HashSet::from([constructed_var1, constructed_var2]);

        let result: HashSet<String> =
            collect_provided_vars(all_vars.iter().cloned(), &app_var_prefix);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_collecting_environment_variables_none_collected() {
        let app_var_prefix: &str = "APP_PREFIX_";

        let all_vars: [(String, String); 0] = [];

        let expected: HashSet<String> = HashSet::from([]);

        let result: HashSet<String> =
            collect_provided_vars(all_vars.iter().cloned(), &app_var_prefix);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_check_env_no_missing() {
        let app_var_prefix: &str = "APP_PREFIX_";

        let required_vars: [&str; 2] = ["VAR_1", "VAR_2"];
        let provided_vars: HashSet<String> = HashSet::from([
            [app_var_prefix, "VAR_1"].concat(),
            [app_var_prefix, "VAR_2"].concat(),
        ]);

        let expect: Result<()> = Ok(());
        let result: Result<()> = check_env(&required_vars, &provided_vars, app_var_prefix);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_check_env_single_missing() {
        let app_var_prefix: &str = "APP_PREFIX_";

        let required_vars: [&str; 2] = ["VAR_1", "VAR_2"];
        let provided_vars: HashSet<String> = HashSet::from([[app_var_prefix, "VAR_1"].concat()]);
        let missing_vars: [String; 1] = [[app_var_prefix, "VAR_2"].concat()];

        let expect: Result<()> = Err(AppError::new(
            ErrorKind::MissingEnvVars(format!(
                "Missing required environment variables: {}",
                missing_vars.join(", ")
            )),
            None,
        ));
        let result: Result<()> = check_env(&required_vars, &provided_vars, app_var_prefix);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_check_env_multiple_missing() {
        let app_var_prefix: &str = "APP_PREFIX_";

        let required_vars: [&str; 3] = ["VAR_1", "VAR_2", "VAR_3"];
        let provided_vars: HashSet<String> = HashSet::from([[app_var_prefix, "VAR_2"].concat()]);
        let missing_vars: [String; 2] = [
            [app_var_prefix, "VAR_1"].concat(),
            [app_var_prefix, "VAR_3"].concat(),
        ];

        let expect: Result<()> = Err(AppError::new(
            ErrorKind::MissingEnvVars(format!(
                "Missing required environment variables: {}",
                missing_vars.join(", ")
            )),
            None,
        ));
        let result: Result<()> = check_env(&required_vars, &provided_vars, app_var_prefix);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_check_env_no_provided() {
        let app_var_prefix: &str = "APP_PREFIX_";

        let required_vars: [&str; 2] = ["VAR_1", "VAR_2"];
        let provided_vars: HashSet<String> = HashSet::from([]);
        let missing_vars: [String; 2] = [
            [app_var_prefix, "VAR_1"].concat(),
            [app_var_prefix, "VAR_2"].concat(),
        ];

        let expect: Result<()> = Err(AppError::new(
            ErrorKind::MissingEnvVars(format!(
                "Missing required environment variables: {}",
                missing_vars.join(", ")
            )),
            None,
        ));
        let result: Result<()> = check_env(&required_vars, &provided_vars, app_var_prefix);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_check_env_no_required() {
        let app_var_prefix: &str = "APP_PREFIX_";

        let required_vars: [&str; 0] = [];
        let provided_vars: HashSet<String> = HashSet::from([
            [app_var_prefix, "VAR_1"].concat(),
            [app_var_prefix, "VAR_2"].concat(),
        ]);

        let expect: Result<()> = Ok(());
        let result: Result<()> = check_env(&required_vars, &provided_vars, app_var_prefix);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_check_env_no_required_no_provided() {
        let app_var_prefix: &str = "APP_PREFIX_";

        let required_vars: [&str; 0] = [];
        let provided_vars: HashSet<String> = HashSet::from([]);

        let expect: Result<()> = Ok(());
        let result: Result<()> = check_env(&required_vars, &provided_vars, app_var_prefix);

        assert_eq!(expect, result);
    }
}

use std::collections::HashSet;
use std::env;

const ENV_VAR_PREFIX: &str = "RRAA_";
const REQUIRED_ENV_VARS: &[&str] = &["DB_URL"];

pub fn load_app_env() {
    println!("Loading environment file contents...");

    load_env_file("./environment/.env");

    let provided_vars: HashSet<String> = collect_provided_vars();

    check_required_vars(&provided_vars);
    print_extra_vars(&provided_vars);

    println!("All required environment variables are set.");
}

fn load_env_file(path: &str) {
    if let Err(err) = dotenvy::from_filename(path) {
        eprintln!("Warning: Could not load .env file: {:?}", err);
    }
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

fn check_required_vars(provided_vars: &HashSet<String>) {
    let missing_vars: Vec<&str> = REQUIRED_ENV_VARS
        .iter()
        .filter(|&&req_var| !provided_vars.contains(req_var))
        .copied()
        .collect();

    if !missing_vars.is_empty() {
        eprintln!(
            "Error: Missing required environment variables: {:?}",
            missing_vars.join(", ")
        );
        std::process::exit(1);
    }
}

fn print_extra_vars(provided_vars: &HashSet<String>) {
    let extra_vars: Vec<String> = provided_vars
        .iter()
        .filter(|var| !REQUIRED_ENV_VARS.contains(&var.as_str()))
        .cloned()
        .collect();

    if !extra_vars.is_empty() {
        println!(
            "Note: Extra environment variables with prefix '{}' found: {:?}",
            ENV_VAR_PREFIX,
            extra_vars.join(", ")
        );
    }
}

use colored::Colorize;
use log::info;
use std::env;

pub mod aws_connectors {
    pub mod aws_s3_bucket_handler;
}

pub fn are_env_vars_set(env_var_names: &[&str]) -> bool {
    let mut all_set = true;
    for &env_var_name in env_var_names {
        match env::var(env_var_name) {
            Ok(value) => {
                // info!("{} is set to: {}", env_var_name, value);
            }
            Err(_) => {
                let mut colored_string: colored::ColoredString;
                colored_string = format!("{} is not set.", env_var_name).red();
                info!("{}", colored_string);

                all_set = false;
            }
        }
    }
    all_set
}
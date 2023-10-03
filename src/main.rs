use aws_sdk_s3::{Client, Error};
use std::env;

fn are_env_vars_set(env_var_names: &[&str]) -> bool {
    let mut all_set = true;
    for &env_var_name in env_var_names {
        match env::var(env_var_name) {
            Ok(value) => {
                // println!("{} is set to: {}", env_var_name, value);
            }
            Err(_) => {
                println!("{} is not set.", env_var_name);
                all_set = false;
            }
        }
    }
    all_set
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define a list of environment variable names to check
    let env_vars_to_check = [ "AWS_ACCESS_KEY_ID",
                                         "AWS_SECRET_ACCESS_KEY",
                                         "AWS_DEFAULT_REGION",
                                         "AWS_ENDPOINT_URL" ];

    // Call the function to check if the environment variables are set
    if are_env_vars_set(&env_vars_to_check) {
        println!("All environment variables are set.");
    } else {
        println!("Some or all environment variables are not set.");
    }

    // Get default credentials
    let config = aws_config::load_from_env().await;

    // Create an S3 client
    let s3 = Client::new(&config);

    show_buckets(&s3).await?;

    Ok(())
}


async fn show_buckets(s3: &Client) -> Result<(), Error> {
    // List the first page of buckets in the account
    let response = s3.list_buckets().send().await?;

    // Check if the response returned any buckets
    if let Some(buckets) = response.buckets() {
        // Print each bucket name out
        for bucket in buckets {
            println!("bucket name: {}", bucket.name().unwrap());
        }
    } else {
        println!("You don't have any buckets!");
    }
    Ok(())
}

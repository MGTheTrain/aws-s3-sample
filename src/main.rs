use aws_sdk_s3::{Client, Error, operation::{create_bucket::{CreateBucketOutput, CreateBucketError}, put_object::{PutObjectOutput, PutObjectError}, get_object::{GetObjectOutput, GetObjectError}}, error::SdkError, types::{BucketLocationConstraint, CreateBucketConfiguration}, primitives::ByteStream};
use std::{io::{Write, self}, env, path::Path}; // bring trait into scope
use std::fs;

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

    // Create an client client
    let client = Client::new(&config);

    let mut region = String::from("eu-central-1");
    match env::var("AWS_DEFAULT_REGION") {
        Ok(value) => {
            // println!("Value of {} is: {}", region, value);
            region = value;
        }
        Err(_) => {
            println!("{} is not set.", region);
        }
    }
    let bucket_name = "mg-test-bucket-v25"; 
    
    let key = "sample.txt";
    let file_name = "sample.txt";
    let file_path = "temp/sample-copy.txt";

    show_buckets(&client).await?;

    // create_bucket(&client, &bucket_name, &region).await?;
    println!("Created bucket with name {}", bucket_name);
    
    upload_object(&client, &bucket_name, &file_name, &key).await?;
    println!("Uploaded file {} with object name {} to bucket with name {}", file_name, key, bucket_name);

    // Download
    let get_object_output = get_object(&client, &bucket_name, &key).await?;
    let data = get_object_output.body.collect().await.unwrap().into_bytes();
    
    let contents = std::str::from_utf8(&data).unwrap(); // Note that this code assumes that the files are utf8 encoded plain text format.
    println!("Key: {key}, Contents: {contents}");

    write_bytes_to_file(&data, &file_path).await; 

    remove_object(&client, &bucket_name, &key).await?;

    delete_bucket(&client, &bucket_name).await?;

    Ok(())
}

// Helper methds

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

async fn show_buckets(client: &Client) -> Result<(), Error> {
    // List the first page of buckets in the account
    let response = client.list_buckets().send().await?;

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

async fn create_bucket(
    client: &Client,
    bucket_name: &str,
    region: &str) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await
}

async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(Path::new(file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
}

async fn get_object(
    client: &Client,
    bucket_name: &str,
    key: &str,) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
    client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await   
}

async fn write_bytes_to_file(
    bytes: &[u8],
    file_path: &str,) -> Result<(), io::Error> {      
    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .open(file_path)?;

    file.write_all(&bytes)?;

    Ok(())
}

async fn remove_object(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    println!("Object deleted.");

    Ok(())
}

async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<(), Error> {
    client.delete_bucket().bucket(bucket_name).send().await?;
    println!("Bucket deleted");
    Ok(())
}
use aws_sdk_s3::{
    error::SdkError,
    operation::{
        create_bucket::{CreateBucketError, CreateBucketOutput},
        delete_object,
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    primitives::ByteStream,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
    Client, Error,
};
use log::info;
use std::fs;
use std::{
    env, error,
    io::{self, Write},
    path::Path,
}; 

use clap::{Parser, Subcommand};

use colored::Colorize;

#[derive(Parser, Debug)]
#[clap(
    author = "MGTheTrain",
    version = "1.0.0",
    about = "A Cli tool enabling blob operations (deletion, upload and download of blobs) and bucket operations (show, create or delete buckets) with AWS S3 buckets."
)]
struct Cli {
    #[clap(subcommand)]
    operation: AWSS3BucketOperation,
}

#[derive(Debug, Subcommand)]
enum AWSS3BucketOperation {
    /// Create bucket operation
    CreateBucket {},
    /// Delete bucket operation
    ShowBucket {},
    /// Delete bucket operation
    DeleteBucket {},
    /// Upload blob operation arguments
    UploadBlob {
        /// the blob name (equivalent to the S3 Bucket key)
        #[clap(short, long)]
        blob_name: Option<String>,
        /// the file path of the blob to be uploaded
        #[clap(short, long)]
        upload_file_path: Option<String>,
    },
    /// Download blob operation arguments
    DownloadBlob {
        /// the blob name (equivalent to the S3 Bucket key)
        #[clap(short, long)]
        blob_name: Option<String>,
        /// the file path in which the blob should be downloaded
        #[clap(short, long)]
        download_file_path: Option<String>,
    },
    /// Delete blob operation arguments
    DeleteBlob {
        /// the blob name (equivalent to the S3 Bucket key)
        #[clap(short, long)]
        blob_name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let mut colored_string: colored::ColoredString;

    let env_file_path = "secrets.cfg";
    dotenv::from_path(env_file_path).ok();

    let env_vars_to_check = [
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "AWS_DEFAULT_REGION",
        "AWS_ENDPOINT_URL",
        "AWS_BUCKET_NAME",
    ];

    if are_env_vars_set(&env_vars_to_check) {
        colored_string = "All environment variables are set.".blue();
        info!("{}", colored_string);
    } else {
        colored_string = "Error: Some or all environment variables are not set.".red();
        panic!("{}", colored_string);
    }

    let config = aws_config::load_from_env().await;

    let client = Client::new(&config);

    let mut region = String::from("eu-central-1");
    colored_string = "Error: AWS_DEFAULT_REGION environment variable expected".red();
    region = std::env::var("AWS_DEFAULT_REGION").expect(&colored_string.to_string());

    colored_string = "Error: AWS_BUCKET_NAME environment variable expected".red();
    let bucket_name = std::env::var("AWS_BUCKET_NAME").expect(&colored_string.to_string());

    let args = Cli::parse();

    match &args.operation {
        AWSS3BucketOperation::CreateBucket {} => {
            create_bucket(&client, &bucket_name, &region).await?;
            colored_string = format!("Created bucket with name {}", bucket_name).blue();
            info!("{}", colored_string);
        }
        AWSS3BucketOperation::ShowBucket {} => {
            show_buckets(&client).await?;
        }
        AWSS3BucketOperation::DeleteBucket {} => {
            delete_bucket(&client, &bucket_name).await?;
        }
        AWSS3BucketOperation::UploadBlob {
            blob_name,
            upload_file_path,
        } => {
            upload_object(
                &client,
                &bucket_name,
                &upload_file_path.clone().unwrap(),
                &blob_name.clone().unwrap(),
            )
            .await?;
            colored_string = format!(
                "Uploaded file {} with object name {} to bucket {}",
                upload_file_path.clone().unwrap(),
                &blob_name.clone().unwrap(),
                bucket_name
            )
            .blue();
            info!("{}", colored_string);
        }
        AWSS3BucketOperation::DownloadBlob {
            blob_name,
            download_file_path,
        } => {
            let get_object_output =
                get_object(&client, &bucket_name, &blob_name.clone().unwrap()).await?;
            let data = get_object_output.body.collect().await.unwrap().into_bytes();
            write_bytes_to_file(&data, &download_file_path.clone().unwrap()).await?;

            colored_string = format!(
                "Downloaded file {} with object name {} from bucket {}",
                download_file_path.clone().unwrap(),
                &blob_name.clone().unwrap(),
                bucket_name
            )
            .blue();
            info!("{}", colored_string);
        }
        AWSS3BucketOperation::DeleteBlob { blob_name } => {
            remove_object(&client, &bucket_name, &blob_name.clone().unwrap()).await?;
        }
        _ => {
            colored_string = "Error: Operation not supported".red();
            panic!("{}", colored_string)
        }
    }

    Ok(())
}

fn are_env_vars_set(env_var_names: &[&str]) -> bool {
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

async fn show_buckets(client: &Client) -> Result<(), Error> {
    let response = client.list_buckets().send().await?;

    if let Some(buckets) = response.buckets() {
        for bucket in buckets {
            let mut colored_string: colored::ColoredString;
            colored_string = format!("Bucket name: {}", bucket.name().unwrap()).blue();
            info!("{}", colored_string);
        }
    } else {
        let mut colored_string: colored::ColoredString;
        colored_string = "You don't have any buckets!".red();
        info!("{}", colored_string);
    }
    Ok(())
}

async fn create_bucket(
    client: &Client,
    bucket_name: &str,
    region: &str,
) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
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
    key: &str,
) -> Result<GetObjectOutput, SdkError<GetObjectError>> {
    client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
}

async fn write_bytes_to_file(bytes: &[u8], file_path: &str) -> Result<(), io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
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

    let mut colored_string: colored::ColoredString;
    colored_string = format!("Object {} deleted from {} bucket.", key, bucket).blue();
    info!("{}", colored_string);

    Ok(())
}

async fn delete_bucket(client: &Client, bucket_name: &str) -> Result<(), Error> {
    client.delete_bucket().bucket(bucket_name).send().await?;

    let mut colored_string: colored::ColoredString;
    colored_string = format!("Bucket {} deleted", bucket_name).blue();
    info!("{}", colored_string);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use aws_sdk_s3::{Client, Error};
    use colored::Colorize;
    use log::info;

    use crate::{
        are_env_vars_set, create_bucket, delete_bucket, get_object, remove_object, show_buckets,
        upload_object, write_bytes_to_file,
    };

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_aws_s3_storage_methods() -> Result<(), Error> {
        env_logger::init();
        let mut colored_string: colored::ColoredString;

        let env_file_path = "secrets.cfg";
        dotenv::from_path(env_file_path).ok();

        let env_vars_to_check = [
            "AWS_ACCESS_KEY_ID",
            "AWS_SECRET_ACCESS_KEY",
            "AWS_DEFAULT_REGION",
            "AWS_ENDPOINT_URL",
            "AWS_BUCKET_NAME",
        ];

        if are_env_vars_set(&env_vars_to_check) {
            colored_string = "All environment variables are set.".blue();
            info!("{}", colored_string);
        } else {
            colored_string = "Error: Some or all environment variables are not set.".red();
            panic!("{}", colored_string);
        }

        let config = aws_config::load_from_env().await;

        let client = Client::new(&config);

        let mut region = String::from("eu-central-1");
        colored_string = "Error: AWS_DEFAULT_REGION environment variable expected".red();
        region = std::env::var("AWS_DEFAULT_REGION").expect(&colored_string.to_string());

        colored_string = "Error: AWS_BUCKET_NAME environment variable expected".red();
        let bucket_name = std::env::var("AWS_BUCKET_NAME").expect(&colored_string.to_string());

        let key = "sample.txt";
        let file_name = "sample.txt";
        let file_path = "output/sample-copy.txt";

        assert!(show_buckets(&client).await.is_ok());
        assert!(create_bucket(&client, &bucket_name, &region).await.is_ok());
        colored_string = format!("Created bucket with name {}", bucket_name).blue();
        info!("{}", colored_string);

        assert!(upload_object(&client, &bucket_name, &file_name, &key)
            .await
            .is_ok());
        colored_string = format!(
            "Uploaded file {} with object name {} to bucket {}",
            file_name, key, bucket_name
        )
        .blue();
        info!("{}", colored_string);

        // Download
        let get_object_output = get_object(&client, &bucket_name, &key).await;
        assert!(get_object_output.is_ok());
        let data = get_object_output?
            .body
            .collect()
            .await
            .unwrap()
            .into_bytes();

        let contents = std::str::from_utf8(&data).unwrap();

        assert!(write_bytes_to_file(&data, &file_path).await.is_ok());
        colored_string = format!(
            "Downloaded file {} with object name {} from bucket {}",
            file_name, key, bucket_name
        )
        .blue();
        info!("{}", colored_string);
        assert!(remove_object(&client, &bucket_name, &key).await.is_ok());
        assert!(delete_bucket(&client, &bucket_name).await.is_ok());

        Ok(())
    }
}

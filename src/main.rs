// The MIT License
//
// Copyright (c) 2024 MGTheTrain
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Maintainers:
// - MGTheTrain
//
// Contributors:
// - TBD

use log::info;
use std::error;

use clap::{Parser, Subcommand};
use colored::Colorize;

use common_modules::are_env_vars_set;
use common_modules::aws_connectors::aws_s3_bucket_handler::AwsS3BucketHandler;

#[derive(Parser, Debug)]
#[clap(
    author = "MGTheTrain",
    version = "1.0.0",
    about = "A Cli tool enabling blob operations (deletion, upload and download of blobs) and bucket operations (show, create or delete buckets) with AWS S3 buckets."
)]
struct Cli {
    #[clap(subcommand)]
    operation: AwsS3BucketOperation,
}

#[derive(Debug, Subcommand)]
enum AwsS3BucketOperation {
    /// Create bucket operation
    CreateBucket {},
    /// Show bucket operation
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
        "AWS_REGION",
        "AWS_BUCKET_NAME",
    ];

    if are_env_vars_set(&env_vars_to_check) {
        colored_string = "All environment variables are set.".blue();
        info!("{}", colored_string);
    } else {
        colored_string = "Error: Some or all environment variables are not set.".red();
        panic!("{}", colored_string);
    }

    colored_string = "Error: AWS_REGION environment variable expected".red();
    let region = std::env::var("AWS_REGION").expect(&colored_string.to_string()); // e.g. eu-central-1

    colored_string = "Error: AWS_BUCKET_NAME environment variable expected".red();
    let bucket_name = std::env::var("AWS_BUCKET_NAME").expect(&colored_string.to_string());

    let aws_s3_bucket_handler = AwsS3BucketHandler::new(&bucket_name, String::from(region)).await?;

    let args = Cli::parse();

    match &args.operation {
        AwsS3BucketOperation::CreateBucket {} => {
            aws_s3_bucket_handler.create_bucket().await?;
        }
        AwsS3BucketOperation::ShowBucket {} => {
            aws_s3_bucket_handler.show_buckets().await?;
        }
        AwsS3BucketOperation::DeleteBucket {} => {
            aws_s3_bucket_handler.delete_bucket().await?;
        }
        AwsS3BucketOperation::UploadBlob {
            blob_name,
            upload_file_path,
        } => {
            aws_s3_bucket_handler
                .upload_blob(
                    &blob_name.clone().unwrap(),
                    &upload_file_path.clone().unwrap(),
                )
                .await?;
        }
        AwsS3BucketOperation::DownloadBlob {
            blob_name,
            download_file_path,
        } => {
            aws_s3_bucket_handler
                .download_blob(
                    &blob_name.clone().unwrap(),
                    &download_file_path.clone().unwrap(),
                )
                .await?;
        }
        AwsS3BucketOperation::DeleteBlob { blob_name } => {
            aws_s3_bucket_handler
                .delete_blob(&blob_name.clone().unwrap())
                .await?;
        }
        _ => {
            colored_string = "Error: Operation not supported".red();
            panic!("{}", colored_string)
        }
    }

    Ok(())
}

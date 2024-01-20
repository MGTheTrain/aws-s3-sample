# rust-aws-s3-bucket-handler

## Table of Contents

- [Summary](#summary)
- [References](#references)
- [How to use](#how-to-use)

## Summary

Repository demonstrating how to manage blobs in AWS S3 service buckets with Rust and required third-party crates.


## References

- [The AWS SDK for Rust ](https://www.serverlessguru.com/blog/aws-sdk-for-rust-getting-started)
- [Amazon S3 examples using SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html)

## How to use

### Precondition

Either 
- AWS can be utilized for deploying AWS S3 buckets or 
- a localstack docker container can be utilized. 

In order to ramp up the localstack container execute:

```bash
cd devops/docker-compose
sudo docker compose up -d --build
```

**NOTE**: experiments on Windows 10 OS with Virtual Box enabled docker and a running localstack container has failed

### Build and run compiled source code 

Create from the [secrets.template.cfg](./templates/secrets.template.cfg) in the [templates folder](./templates/) a `secrets.cfg` file in the project root directory and replace the `PLACEHOLDER_*` values. The [test_aws_blob_handler.rs](./test/test_aws_blob_handler.rs) and [main.rs](./src/main.rs) will export the environment variables trough the `secrets.cfg` file.
Afterwards execute the following to run the tests:

```bash
cargo test
# for more logs execute
RUST_LOG=info cargo test
```

Build and run the executable binary with:

```bash
# Precondition for further actions on Aws S3 buckets - Example create bucket: 
RUST_LOG=info cargo run -- create-bucket

# Example blob upload: 
RUST_LOG=info cargo run -- upload-blob -b blob.txt  -u assets/sample.txt
RUST_LOG=info cargo run -- upload-blob --blob-name blob.txt --upload-file-path assets/sample.txt 

# Example blob download: 
RUST_LOG=info cargo run -- download-blob -b blob.txt -d output/download.txt
RUST_LOG=info cargo run -- download-blob --blob-name blob.txt --download-file-path "output/download.txt"

# Example blob delete: 
RUST_LOG=info cargo run -- delete-blob -b blob.txt
RUST_LOG=info cargo run -- delete-blob --blob-name blob.txt

# Example show bucket: 
RUST_LOG=info cargo run -- show-bucket

# Example delete bucket: 
RUST_LOG=info cargo run -- delete-bucket

#####################################################################################################################
# Running the executable without cargo  
cargo build
cp target/debug/aws_s3_bucket_handler.exe . # On Windows OS when utilizing Git Bash or WSL
source secrets.cfg
./aws_s3_bucket_handler --help
# Example blob upload (Note: Colored crates console logs might not work on certain terminals): 
RUST_LOG=info ./aws_s3_bucket_handler upload-blob --blob-name blob.txt --upload-file-path assets/sample.txt 
```

### (Optional) Bash scripts for testing out communication with localstack docker container

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash localstack-demo](./devops/docker-compose/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the localstack docker container s3 service. | 

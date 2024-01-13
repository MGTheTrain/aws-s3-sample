# rust-aws-s3-bucket-handler

## Table of Contents

- [Summary](#summary)
- [References](#references)
- [How to use](#how-to-use)
  - [Rust](#rust)
  - [Optional](#optional)
    - [Initiate the Docker Compose cluster to launch a locally running Localstack Docker container](#initiate-the-docker-compose-cluster-to-launch-a-locally-running-localstack-docker-container)
    - [Run tests](#run-tests)


## Summary

Repository demonstrating how to manage blobs in AWS S3 service buckets with Rust and required third-party crates.


## References

- [The AWS SDK for Rust ](https://www.serverlessguru.com/blog/aws-sdk-for-rust-getting-started)
- [Amazon S3 examples using SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html)

## How to use

### Rust

The [Rust sample](./src/main.rs) can be started with `cargo run`. Please note that the localstack docker container s3 service can not be used for local blob management tests (upload, download, deletion, metadata retrieval, etc.) on Windows 10 OS. You need to utilize a public s3 service bucket.

Create from the [secrets.template.cfg](./templates/secrets.template.cfg) in the [templates folder](./templates/) a `secrets.cfg` file in the project root directory and replace the `<PLACEHOLDER_*>` values. The [test_aws_blob_handler.rs](./test/test_aws_blob_handler.rs) and [main.rs](./src/main.rs) will export the environment variables trough the `secrets.cfg` file.
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
# Building and running the executable without cargo  
cargo build
cp target/debug/aws_s3_bucket_handler.exe . # On Windows OS when utilizing Git Bash or WSL
source secrets.cfg
./aws_s3_bucket_handler --help
# Example blob upload (Note: Colored crates console logs might not work on certain terminals): 
RUST_LOG=info ./aws_s3_bucket_handler upload-blob --blob-name blob.txt --upload-file-path assets/sample.txt 
```

### Optional

#### Initiate the docker-compose cluster to launch a Localstack docker container

```bash
sudo docker compose up -d --build
```

#### Run tests

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./scripts/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the localstack docker container s3 service. | 

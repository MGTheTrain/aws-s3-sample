# rust-aws-s3-sample

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

The [Rust sample](./src/main.rs) can be started with `cargo run`. Please note that the localstack docker container s3 service can not be used for local blob management tests (up-, download, deletion, metadata retrieval, etc.) on Windows 10 OS. You need to utilize a public s3 service bucket.

Therefore create from the [secrets.template.cfg](./secrets.template.cfg) a `secrets.cfg` file and replace the `<PLACEHOLDER_*>` values.
Afterwards execute the following to run the tests:

```bash
cargo test
# for more logs execute
RUST_LOG=info cargo test
```

Build and run the executable binary with:

```bash
cargo build

# Example blob upload: 
RUST_LOG=info cargo run -- upload-blob -b blob.txt  -u sample.txt
RUST_LOG=info cargo run -- upload-blob --blob-name blob.txt --upload-file-path sample.txt 

# Example blob download: 
RUST_LOG=info cargo run -- download-blob -b blob.txt -d output/download.txt
RUST_LOG=info cargo run -- download-blob --blob-name blob.txt --download-file-path "output/download.txt"

# Example blob delete: 
RUST_LOG=info cargo run -- delete-blob -b blob.txt
RUST_LOG=info cargo run -- delete-blob --blob-name blob.txt

# Example create bucket: 
RUST_LOG=info cargo run -- create bucket
RUST_LOG=info cargo run -- create bucket

# Example show bucket: 
RUST_LOG=info cargo run -- show-bucket
RUST_LOG=info cargo run -- show-bucket

# Example delete bucket: 
RUST_LOG=info cargo run -- delete-bucket
RUST_LOG=info cargo run -- delete-bucket

# or running the executable  
cp target/debug/aws-s3-storage.exe . # On Windows OS when utilizing Git Bash or WSL
source secrets.cfg
./aws-s3-storage --help
# Example blob upload (Note: Colored crates console logs might not work on certain terminals): 
RUST_LOG=info ./aws-s3-storage upload-blob --blob-name blob.txt --upload-file-path sample.txt 
```

### Optional

#### Initiate the Docker Compose cluster to launch a locally running Localstack Docker container

```bash
sudo docker compose up -d --build
```

#### Run tests

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./scripts/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the localstack docker container s3 service. | 

# rust-aws-s3-sample

## Table of Contents

+ [Summary](#summary)
+ [References](#references)
+ [How to use](#how-to-use)

## Summary

Repository demonstrating how to manage blobs in AWS S3 service buckets with Rust and required third-party crates.


## References

- [The AWS SDK for Rust ](https://www.serverlessguru.com/blog/aws-sdk-for-rust-getting-started)
- [Amazon S3 examples using SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html)

## How to use

### Rust

The [Rust sample](./src/main.rs) can be started with `cargo run`. Please note that the localstack docker container s3 service can not be used for local blob management tests (up-, download, deletion, metadata retrieval, etc.) on Windows 10 OS. You need to utilize a public s3 service bucket.

Therefore create from the [secrets.template.cfg](./temp/secrets.template.cfg) a `secrets.cfg` file and replace the `<PLACEHOLDER_*>` values.
Afterwards execute the following to run the tests:

```bash
cargo test
# cosidering logs execute
RUST_LOG=info cargo test
```

In order to build and run the main executable run:

```bash
cargo build

# Example upload: 
cargo.exe run -- upload -b blob.txt  -u sample.txt
cargo.exe run -- upload --blob-name blob.txt --upload-file-path sample.txt 

# Example download: 
cargo.exe run -- download -b blob.txt -d output/download.txt
cargo.exe run -- download --blob-name blob.txt --download-file-path "output/download.txt"

# Example delete: 
cargo.exe run -- delete -b blob.txt
cargo.exe run -- delete --blob-name blob.txt

# or running the executable  
cp target/debug/aws-s3-storage .
source secrets.cfg
./aws-s3-storage --help
# Example upload (Note: Colored crates console logs might not work on certain terminals): 
./aws-s3-storage upload --blob-name blob.txt --upload-file-path sample.txt 
```

### Optional

#### Run the docker compose cluster to have an localstack docker container locally running

```bash
sudo docker compose up -d --build
```

#### Run tests

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./scripts/bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the localstack docker container s3 service. | 

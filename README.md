# aws-s3-sample

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

**0. Run the docker compose cluster to have an localstack docker container locally running:**

```bash
sudo docker compose up -d --build
```

**1. Run tests**

| Scripting language | Description | 
|----------|----------|
| Bash | Navigate to the [bash scripts](./bash/) folder. Go trough the comments in the Dockerfile and utilize it as a reference for testing blob up- and downloads to/from the localstack docker container s3 service. | 

**2. Run Rust sample**

The [Rust sample](./src/main.rs) can be started with `cargo run`. Please note that the localstack docker container s3 service can not be used for local blob management tests (up-, download, deletion, metadata retrieval, etc.) on Windows 10 OS. You need to utilize a public s3 service bucket.

Therefore create from the [secrets.template.cfg](./temp/secrets.cfg.template) a `secrets.cfg` file and replace the `<PLACEHOLDER_*>` values.
Afterwards execute the following:

```bash
source temp/secrets.cfg
cargo run
```


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

use aws_config::{
    meta::region::{ProvideRegion, RegionProviderChain},
    Region,
};
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
use std::{borrow::Borrow, fs};
use std::{
    io::{self, Write},
    path::Path,
};

use colored::Colorize;

pub struct AwsS3BucketHandler {
    client: Client,
    bucket_name: String,
    region: String,
}

impl AwsS3BucketHandler {
    pub async fn new(
        bucket_name: &str,
        region: String,
    ) -> Result<Self, SdkError<CreateBucketError>> {
        let region_provider = RegionProviderChain::first_try(Region::new(region.clone()));
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Ok(Self {
            client,
            bucket_name: bucket_name.to_owned(),
            region: region.to_owned(),
        })
    }

    pub async fn create_bucket(&self) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
        let constraint = BucketLocationConstraint::from(&self.region as &str);
        let cfg = CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();
        let colored_string =
            format!("About to create bucket with name {}", &self.bucket_name).blue();
        info!("{}", colored_string);
        self.client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(&self.bucket_name)
            .send()
            .await
    }

    pub async fn show_buckets(&self) -> Result<(), Error> {
        let response = self.client.list_buckets().send().await?;

        let buckets = response.buckets();
        for bucket in buckets.iter() {
            let colored_string: colored::ColoredString;
            colored_string = format!("Bucket name: {}", bucket.name().unwrap_or_default()).blue();
            info!("{}", colored_string);
        }

        Ok(())
    }

    pub async fn upload_blob(
        &self,
        blob_name: &str,
        upload_file_path: &str,
    ) -> Result<(), SdkError<PutObjectError>> {
        let body = ByteStream::from_path(Path::new(upload_file_path)).await;
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(blob_name)
            .body(body.unwrap())
            .send()
            .await?;

        let colored_string = format!(
            "Uploaded file {} with object name {} to bucket {}",
            upload_file_path, blob_name, self.bucket_name
        )
        .blue();
        info!("{}", colored_string);

        Ok(())
    }

    pub async fn download_blob(
        &self,
        blob_name: &str,
        download_file_path: &str,
    ) -> Result<(), SdkError<GetObjectError>> {
        let get_object_output = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(blob_name)
            .send()
            .await?;
        let data = get_object_output.body.collect().await.unwrap().into_bytes();
        _ = self.write_bytes_to_file(&data, download_file_path).await;

        let colored_string = format!(
            "Downloaded file {} with object name {} from bucket {}",
            download_file_path, blob_name, self.bucket_name
        )
        .blue();
        info!("{}", colored_string);

        Ok(())
    }

    pub async fn delete_blob(&self, blob_name: &str) -> Result<(), Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(blob_name)
            .send()
            .await?;

        let colored_string = format!(
            "Object {} deleted from {} bucket.",
            blob_name, self.bucket_name
        )
        .blue();
        info!("{}", colored_string);

        Ok(())
    }

    pub async fn delete_bucket(&self) -> Result<(), Error> {
        self.client
            .delete_bucket()
            .bucket(&self.bucket_name)
            .send()
            .await?;

        let colored_string = format!("Bucket {} deleted", self.bucket_name).blue();
        info!("{}", colored_string);

        Ok(())
    }

    pub async fn write_bytes_to_file(
        &self,
        bytes: &[u8],
        file_path: &str,
    ) -> Result<(), io::Error> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)?;

        file.write_all(&bytes)?;

        Ok(())
    }
}

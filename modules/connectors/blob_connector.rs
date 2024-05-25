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


use aws_sdk_s3::{
    error::SdkError,
    operation::{
        create_bucket::{CreateBucketError, CreateBucketOutput},
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    Error,
};
use std::{
    io::{self, Write},
    path::Path,
};
use async_trait::async_trait;

#[async_trait]
pub trait BlobConnector: Send + Sync {
    async fn create_bucket(&self) -> Result<CreateBucketOutput, SdkError<CreateBucketError>>;
    async fn show_buckets(&self) -> Result<(), Error>;
    async fn upload_blob(
        &self,
        blob_name: &str,
        upload_file_path: &str,
    ) -> Result<(), SdkError<PutObjectError>>;
    async fn download_blob(
        &self,
        blob_name: &str,
        download_file_path: &str,
    ) -> Result<(), SdkError<GetObjectError>>;
    async fn delete_blob(&self, blob_name: &str) -> Result<(), Error>;
    async fn delete_bucket(&self) -> Result<(), Error>;
    async fn write_bytes_to_file(
        &self,
        bytes: &[u8],
        file_path: &str,
    ) -> Result<(), io::Error>;
}
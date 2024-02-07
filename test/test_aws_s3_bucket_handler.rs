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

#[cfg(test)]
mod tests {
    use std::env;

    use aws_sdk_s3::Error;
    use colored::Colorize;
    use log::info;

    use common_modules::are_env_vars_set;
    use common_modules::aws_connectors::aws_s3_bucket_handler::AwsS3BucketHandler;

    // In order to run the test execute: `RUST_LOG=info cargo test`
    #[tokio::test]
    async fn test_aws_s3_storage_methods() -> Result<(), Error> {
        env_logger::init();
        let mut colored_string: colored::ColoredString;

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

        for (key, value) in env::vars() {
            colored_string = format!("{}: {}", key, value).green();
            info!("{}", colored_string);
        }

        let mut region = String::from("eu-central-1");
        colored_string = "Error: AWS_REGION environment variable expected".red();
        region = std::env::var("AWS_REGION").expect(&colored_string.to_string());

        colored_string = "Error: AWS_BUCKET_NAME environment variable expected".red();
        let bucket_name = std::env::var("AWS_BUCKET_NAME").expect(&colored_string.to_string());

        let blob_name = "sample.txt"; // AWS S3 Bucket terminology would be "key" for blob_name
        let upload_file_path = "test/assets/sample.txt";
        let download_file_path = "test/output/sample-copy.txt";

        let aws_s3_bucket_handler =
            AwsS3BucketHandler::new(&bucket_name, String::from(region)).await?;
        aws_s3_bucket_handler.create_bucket().await?;
        aws_s3_bucket_handler.show_buckets().await?;
        aws_s3_bucket_handler
            .upload_blob(&blob_name, &upload_file_path)
            .await?;

        aws_s3_bucket_handler
            .download_blob(&blob_name, &download_file_path)
            .await?;
        aws_s3_bucket_handler.delete_blob(&blob_name).await?;
        aws_s3_bucket_handler.delete_bucket().await?;

        Ok(())
    }
}

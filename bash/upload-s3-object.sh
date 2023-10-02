#!/bin/bash

aws configure set aws_access_key_id test
aws configure set aws_secret_access_key test
aws configure set default.region us-east-1
# AWS S3: Port 4572 (HTTP) and Port 4573 (HTTPS) - Used to emulate Amazon S3.
aws configure set default.endpoint_url http://localstack:4566
# Option 1a: Set the AWS_PAGER environment variable to an empty string
export AWS_PAGER=""
# Then run your AWS CLI command
aws s3api create-bucket --bucket mybucket
# # Option 1b: Use the --no-cli-pager flag
# aws s3api create-bucket --bucket mybucket2 --no-cli-pager
aws s3 cp sample.txt s3://mybucket/sample.txt
aws s3 ls s3://mybucket/
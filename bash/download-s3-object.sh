#!/bin/bash

aws configure set aws_access_key_id test
aws configure set aws_secret_access_key test
aws configure set default.region us-east-1
# AWS S3: Port 4572 (HTTP) and Port 4573 (HTTPS) - Used to emulate Amazon S3.
aws configure set default.endpoint_url http://localstack:4566
# Option 1a: Set the AWS_PAGER environment variable to an empty string
export AWS_PAGER=""
# Single download
aws s3 cp s3://mybucket/sample.txt sample-download.txt
# Multiple download
# aws s3 cp s3://bucket-name/*.txt txt-folder/
# Multiple recursive download
# aws s3 cp s3://bucket-name/folder-name local-destination-folder/ --recursive

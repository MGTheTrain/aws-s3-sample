output "bucket_name_list" {
    value = [for s3_bucket in aws_s3_bucket.this : s3_bucket.bucket]
}

output "bucket_domain_name_list" {
    value = [for s3_bucket in aws_s3_bucket.this : s3_bucket.bucket_domain_name]
}
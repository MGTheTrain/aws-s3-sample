resource "aws_s3_bucket" "this" {
  bucket = local.bucket_names[count.index]
  count  = var.number_of_buckets
  tags   = local.tags
}
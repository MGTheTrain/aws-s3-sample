locals {
  tags = {
    env         = "${var.environment}",
    team        = "${var.team}",
    owner       = "MGTheTrain",
    project     = "rust-aws-s3-bucket-handler",
    app-purpose = "Deployment of temporary integration test environment",
    Stage       = "${var.environment}"
  }
  bucket_names = [for i in range(var.number_of_buckets) : format("%s%sb%03d", var.digital_product_affix, var.environment, i + 1)]
}
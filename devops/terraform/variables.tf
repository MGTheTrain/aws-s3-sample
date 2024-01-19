# AWS
variable "digital_product_affix" {
  default     = "rasbh"
  description = "The digital product affix."
  type        = string
}

variable "environment" {
  default     = "sbx"
  description = "The environment."
  type        = string

  validation {
    condition     = can(regex("^(sbx|dev|qas|prd)$", var.environment))
    error_message = "Invalid input. Options: \"sbx\", \"dev\", \"qas\", \"prd\"."
  }
}

variable "team" {
  default     = "MG Innovators"
  description = "The team used for tagging resource groups and resources."
  type        = string
}

variable "number_of_buckets" {
  default     = 1
  description = "The total number of S3 buckets to deploy."
  type        = number
}
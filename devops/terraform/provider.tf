provider "aws" {}

terraform {
  experiments = [variable_validation]
}
provider "aws" {
  region  = "eu-west-3"
}

terraform {
  backend "s3" {
  }
}

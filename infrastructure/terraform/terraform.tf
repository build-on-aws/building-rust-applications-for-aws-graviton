provider "aws" {
  region  = "us-west-2"
  profile = "gvlab"
}

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.13.1"
    }
  }
}

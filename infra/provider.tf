terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }

  backend "s3" {
    bucket         = "tf-state-20230722071359242500000001"
    dynamodb_table = "tf-state-20230722071359242500000001"
    key            = "garrettdavis-dev"
    region         = "us-west-2"
    encrypt        = true
    kms_key_id     = "alias/terraform-state"
  }
}

provider "aws" {
  region = "us-west-2"
}

provider "aws" {
  region = "us-east-1"
  alias  = "us-east-1"
}

terraform {
  backend "s3" {
    bucket         = "cs2-server-tf-state"
    key            = "cs2-server/terraform.tfstate"
    region         = "eu-west-1"
    dynamodb_table = "terraform-locks"
    encrypt        = true
  }
}

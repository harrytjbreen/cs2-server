variable "aws_region" {
  type        = string
  description = "AWS Region"
  default     = "eu-west-2"
}

variable "project_name" {
  type = string
}

variable "environment" {
  type = string
}

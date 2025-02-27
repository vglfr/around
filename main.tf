terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.88.0"
    }
  }
}

provider "aws" {
  profile = "default"
  region  = "us-east-1"
}

data "aws_vpc" "default" {
  default = true
}

data "aws_security_group" "default" {
  name = "default"
}

resource "aws_security_group" "ssh" {
  vpc_id = data.aws_vpc.default.id

  ingress {
    from_port = "22"
    to_port = "22"
    protocol = "tcp"
    cidr_blocks = ["73.250.25.0/24"]
  }
}

resource "aws_security_group" "http" {
  vpc_id = data.aws_vpc.default.id

  ingress {
    from_port = "8080"
    to_port = "8080"
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port = "5432"
    to_port = "5432"
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_instance" "tmp" {
  ami = "ami-029f33a91738d30e9"
  instance_type = "t2.small"

  user_data = <<-EOT
    #!/bin/sh
    sudo apt update
    sudo apt install docker.io docker-compose -y
    git clone https://github.com/vglfr/around
  EOT

  vpc_security_group_ids = [
    data.aws_security_group.default.id,
    resource.aws_security_group.http.id,
    resource.aws_security_group.ssh.id,
  ]
}

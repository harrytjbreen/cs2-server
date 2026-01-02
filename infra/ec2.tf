data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

data "aws_ami" "ubuntu" {
  most_recent = true
  owners      = ["099720109477"] # Canonical

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]
  }
}

resource "aws_security_group" "cs2" {
  name        = "${var.project_name}-${var.environment}-cs2"
  description = "CS2 server"
  vpc_id      = data.aws_vpc.default.id

  # Game traffic
  ingress {
    description = "CS2 Game UDP"
    from_port   = 27015
    to_port     = 27015
    protocol    = "udp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Steam client traffic (important)
  ingress {
    description = "Steam UDP"
    from_port   = 27005
    to_port     = 27005
    protocol    = "udp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Query / GOTV / buffer range
  ingress {
    description = "CS2 UDP Range"
    from_port   = 27015
    to_port     = 27020
    protocol    = "udp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # RCON
  ingress {
    description = "RCON TCP"
    from_port   = 27015
    to_port     = 27015
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "${var.project_name}-${var.environment}-cs2"
  }
}

resource "aws_iam_role" "ssm" {
  name = "${var.project_name}-${var.environment}-ssm"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        Service = "ec2.amazonaws.com"
      }
      Action = "sts:AssumeRole"
    }]
  })
}

resource "aws_iam_policy" "cs2_secrets" {
  name = "cs2-secrets-access"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = "secretsmanager:GetSecretValue"
        Resource = aws_secretsmanager_secret.cs2.arn
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "ssm" {
  role       = aws_iam_role.ssm.name
  policy_arn = aws_iam_policy.cs2_secrets.arn
}

resource "aws_iam_role_policy_attachment" "ssm_core" {
  role       = aws_iam_role.ssm.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_instance_profile" "ssm" {
  name = aws_iam_role.ssm.name
  role = aws_iam_role.ssm.name
}

resource "aws_instance" "cs2" {
  ami           = data.aws_ami.ubuntu.id
  instance_type = "t3.small"

  subnet_id              = data.aws_subnets.default.ids[0]
  vpc_security_group_ids = [aws_security_group.cs2.id]

  # SSH intentionally omitted (SSM-ready)
  key_name = null

  iam_instance_profile = aws_iam_instance_profile.ssm.name

  root_block_device {
    volume_size           = 70
    volume_type           = "gp3"
    delete_on_termination = true
  }

  user_data = file("${path.module}/user_data.sh")

  tags = {
    Name = "${var.project_name}-${var.environment}-cs2"
  }
}

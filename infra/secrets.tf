resource "aws_secretsmanager_secret" "cs2" {
  name        = "cs2/prod"
  description = "CS2 server runtime secrets (RCON, Steam GSLT)"

  recovery_window_in_days = 0
}

resource "aws_dynamodb_table" "rust-link-shortener" {
  name         = "url_shortener"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "short_url"

  attribute {
    name = "short_url"
    type = "S"
  }

  tags = {
    Name = "url_shortener"
  }
}

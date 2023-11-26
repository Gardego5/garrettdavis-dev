resource "aws_s3_bucket" "static" {
  bucket = "${local.name_prefix}-static"

  tags = {
    NamePrefix = local.name_prefix
  }
}

resource "aws_s3_bucket_website_configuration" "static" {
  bucket = aws_s3_bucket.static.id
  index_document {
    suffix = "index.html"
  }
  error_document {
    key = "error.html"
  }
}

resource "aws_s3_bucket_cors_configuration" "static" {
  bucket = aws_s3_bucket.static.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET"]
    allowed_origins = ["*"]
    expose_headers  = ["ETag"]
    max_age_seconds = 3000
  }
}

resource "aws_s3_bucket_acl" "static" {
  bucket     = aws_s3_bucket.static.id
  acl        = "public-read"
  depends_on = [aws_s3_bucket_ownership_controls.static]
}

resource "aws_s3_bucket_ownership_controls" "static" {
  bucket = aws_s3_bucket.static.id
  rule {
    object_ownership = "BucketOwnerPreferred"
  }
  depends_on = [aws_s3_bucket_public_access_block.static]
}

resource "aws_s3_bucket_public_access_block" "static" {
  bucket                  = aws_s3_bucket.static.id
  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_policy" "static" {
  bucket     = aws_s3_bucket.static.id
  policy     = data.aws_iam_policy_document.static.json
  depends_on = [aws_s3_bucket_public_access_block.static]
}

data "aws_iam_policy_document" "static" {
  statement {
    sid       = "AddPerm"
    actions   = ["s3:GetObject"]
    resources = ["${aws_s3_bucket.static.arn}/*"]
    effect    = "Allow"
    principals {
      type        = "AWS"
      identifiers = ["*"]
    }
  }
}

locals {
  content_types = {
    "css"  = "text/css",
    "html" = "text/html",
    "jpg"  = "image/jpeg",
    "js"   = "application/javascript",
  }
}

resource "aws_s3_object" "static_files" {
  for_each     = fileset("../static/", "**/*.{${join(",", keys(local.content_types))}}")
  bucket       = aws_s3_bucket.static.id
  key          = "static/${each.value}"
  source       = "../static/${each.value}"
  etag         = filemd5("../static/${each.value}")
  content_type = local.content_types[split(".", each.value)[length(split(".", each.value)) - 1]]

  tags = {
    NamePrefix = local.name_prefix
  }
}

resource "aws_s3_object" "built_static_files" {
  for_each     = fileset("../dist/", "**/*.{${join(",", keys(local.content_types))}}")
  bucket       = aws_s3_bucket.static.id
  key          = "static/${each.value}"
  source       = "../dist/${each.value}"
  etag         = filemd5("../dist/${each.value}")
  content_type = local.content_types[split(".", each.value)[length(split(".", each.value)) - 1]]

  tags = {
    NamePrefix = local.name_prefix
  }
}

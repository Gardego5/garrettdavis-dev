resource "aws_lambda_function" "function" {
  function_name    = local.name_prefix
  filename         = data.archive_file.lambda.output_path
  source_code_hash = data.archive_file.lambda.output_base64sha256
  role             = aws_iam_role.lambda.arn

  architectures = ["arm64"]
  handler       = "bootstrap"
  memory_size   = 128
  runtime       = "provided.al2"
  timeout       = 3

  depends_on = [
    aws_iam_role.lambda,
    aws_cloudwatch_log_group.lambda,
  ]
}

data "archive_file" "lambda" {
  type             = "zip"
  source_dir       = "${path.module}/../lambda"
  output_file_mode = "0666"
  output_path      = "../dist.zip"
}

resource "aws_cloudwatch_log_group" "lambda" {
  name              = "/aws/lambda/${local.name_prefix}"
  retention_in_days = 7
}

resource "aws_iam_role" "lambda" {
  name               = local.name_prefix
  assume_role_policy = data.aws_iam_policy_document.lambda_asume_role.json
}

data "aws_iam_policy_document" "lambda_asume_role" {
  statement {
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role_policy" "lambda" {
  name   = local.name_prefix
  role   = aws_iam_role.lambda.id
  policy = data.aws_iam_policy_document.lambda.json
}

data "aws_iam_policy_document" "lambda" {
  statement {
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
    ]
    resources = [
      aws_cloudwatch_log_group.lambda.arn,
      "${aws_cloudwatch_log_group.lambda.arn}:*",
    ]
  }
}

resource "aws_lambda_function_url" "lambda" {
  function_name      = aws_lambda_function.function.function_name
  authorization_type = "NONE"
  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["*"]
    allow_headers     = ["*"]
    expose_headers    = ["*"]
  }
}

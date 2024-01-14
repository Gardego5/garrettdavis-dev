output "s3_website_endpoint" {
  value = aws_s3_bucket_website_configuration.static.website_endpoint
}

output "lambda_function_url" {
  value = aws_lambda_function_url.lambda.function_url
}


resource "aws_cloudfront_origin_access_identity" "origin_access_identity" {
  comment = "garrettdavis.dev"
}

resource "aws_cloudfront_distribution" "s3_distribution" {
  origin {
    domain_name = replace(replace(aws_lambda_function_url.lambda.function_url, "https://", ""), "/", "")
    origin_id   = "lambda-${local.name_prefix}"

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  origin {
    domain_name = aws_s3_bucket_website_configuration.static.website_endpoint
    origin_id   = "s3-${local.name_prefix}"

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "http-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  enabled             = true
  is_ipv6_enabled     = true
  comment             = local.name_prefix
  default_root_object = "index.html"
  aliases             = ["garrettdavis.dev", "www.garrettdavis.dev"]

  default_cache_behavior {
    target_origin_id       = "lambda-${local.name_prefix}"
    allowed_methods        = ["POST", "HEAD", "PATCH", "DELETE", "PUT", "GET", "OPTIONS"]
    cached_methods         = ["GET", "HEAD"]
    cache_policy_id        = aws_cloudfront_cache_policy.lambda.id
    viewer_protocol_policy = "redirect-to-https"
    compress               = true
    function_association {
      event_type   = "viewer-request"
      function_arn = aws_cloudfront_function.redirect_to_www.arn
    }
  }

  ordered_cache_behavior {
    path_pattern           = "/static/*"
    target_origin_id       = "s3-${local.name_prefix}"
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    cache_policy_id        = data.aws_cloudfront_cache_policy.caching_optimized.id
    viewer_protocol_policy = "redirect-to-https"
    compress               = true
    function_association {
      event_type   = "viewer-request"
      function_arn = aws_cloudfront_function.redirect_to_www.arn
    }
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn = data.aws_acm_certificate.certificate.arn
    ssl_support_method  = "sni-only"
  }


  tags = {
    NamePrefix = local.name_prefix
  }
}

resource "aws_cloudfront_cache_policy" "lambda" {
  name        = "lambda-${local.name_prefix}"
  min_ttl     = 1
  default_ttl = 50
  max_ttl     = 100
  parameters_in_cache_key_and_forwarded_to_origin {
    cookies_config {
      cookie_behavior = "all"
    }
    headers_config {
      header_behavior = "none"
    }
    query_strings_config {
      query_string_behavior = "none"
    }
  }
}

data "aws_acm_certificate" "certificate" {
  provider    = aws.us-east-1
  domain      = "garrettdavis.dev"
  statuses    = ["ISSUED"]
  most_recent = true
}

resource "aws_cloudfront_function" "redirect_to_www" {
  name    = "redirect-to-www-${local.name_prefix}"
  runtime = "cloudfront-js-1.0"
  code    = <<JS
  function handler(event) {
    var request = event.request;
    if (request.headers.host.value == "garrettdavis.dev") {
      var value = "https://www.garrettdavis.dev" + request.uri;
      return {
        statusCode: 301,
        statusDescription: "Moved Permanently",
        headers: { location: { value } },
      };
    } else {
      return request;
    }
  }
  JS
}


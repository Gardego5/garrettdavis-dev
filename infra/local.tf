locals {
  name_prefix = "garrettdavis-dev"
}

data "aws_cloudfront_cache_policy" "caching_optimized" {
  name = "Managed-CachingOptimized"
}

# Fix init functions for all modules

# Add init function to caching.rs
$cachingContent = Get-Content "services\api-gateway\src\caching.rs" -Raw
$cachingContent = $cachingContent -replace "impl CacheService \{", "/// Initialize cache service`npub async fn init(config: &Config) -> Result<CacheService> {`n    Ok(CacheService::new(config.clone()))`n}`n`nimpl CacheService {"
Set-Content "services\api-gateway\src\caching.rs" $cachingContent

# Add init function to rate_limiting.rs
$rateLimitingContent = Get-Content "services\api-gateway\src\rate_limiting.rs" -Raw
$rateLimitingContent = $rateLimitingContent -replace "impl RateLimiter \{", "/// Initialize rate limiter`npub async fn init(config: &Config) -> Result<RateLimiter> {`n    Ok(RateLimiter::new(config.clone()))`n}`n`nimpl RateLimiter {"
Set-Content "services\api-gateway\src\rate_limiting.rs" $rateLimitingContent

# Add init function to auth.rs
$authContent = Get-Content "services\api-gateway\src\auth.rs" -Raw
$authContent = $authContent -replace "impl AuthService \{", "/// Initialize auth service`npub async fn init(config: &Config) -> Result<AuthService> {`n    Ok(AuthService::new(config.clone()))`n}`n`nimpl AuthService {"
Set-Content "services\api-gateway\src\auth.rs" $authContent

# Add init function to load_balancing.rs
$loadBalancingContent = Get-Content "services\api-gateway\src\load_balancing.rs" -Raw
$loadBalancingContent = $loadBalancingContent -replace "impl LoadBalancer \{", "/// Initialize load balancer`npub async fn init(config: &Config, service_registry: Arc<ServiceDiscoveryService>) -> Result<LoadBalancer> {`n    Ok(LoadBalancer::new(config.clone(), service_registry))`n}`n`nimpl LoadBalancer {"
Set-Content "services\api-gateway\src\load_balancing.rs" $loadBalancingContent

# Add init function to routing.rs
$routingContent = Get-Content "services\api-gateway\src\routing.rs" -Raw
$routingContent = $routingContent -replace "impl RouterService \{", "/// Initialize router service`npub async fn init(config: &Config, load_balancer: Arc<LoadBalancer>, auth_service: Arc<AuthService>, rate_limiter: Arc<RateLimiter>, cache: Arc<CacheService>) -> Result<RouterService> {`n    Ok(RouterService::new(config.clone(), load_balancer, auth_service, rate_limiter, cache))`n}`n`nimpl RouterService {"
Set-Content "services\api-gateway\src\routing.rs" $routingContent

Write-Host "Fixed init functions for all modules"

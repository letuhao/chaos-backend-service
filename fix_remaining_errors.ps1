# Fix remaining compilation errors

# Fix Router type in server.rs
$serverContent = Get-Content "services\api-gateway\src\server.rs" -Raw
$serverContent = $serverContent -replace "use crate::routing::RouterService;", "use axum::Router;"
$serverContent = $serverContent -replace "router: Router,", "router: Router,"
$serverContent = $serverContent -replace "pub fn new\(config: Config, router: Router\)", "pub fn new(config: Config, router: Router)"
$serverContent = $serverContent -replace "pub fn router\(&self\) -> &Router", "pub fn router(&self) -> &Router"
Set-Content "services\api-gateway\src\server.rs" $serverContent

# Fix init functions return types
$authContent = Get-Content "services\api-gateway\src\auth.rs" -Raw
$authContent = $authContent -replace "Ok\(AuthService::new\(config\.clone\(\)\)\)", "AuthService::new(config.clone())"
Set-Content "services\api-gateway\src\auth.rs" $authContent

$rateLimitingContent = Get-Content "services\api-gateway\src\rate_limiting.rs" -Raw
$rateLimitingContent = $rateLimitingContent -replace "Ok\(RateLimiter::new\(config\.clone\(\)\)\)", "RateLimiter::new(config.clone())"
Set-Content "services\api-gateway\src\rate_limiting.rs" $rateLimitingContent

$cachingContent = Get-Content "services\api-gateway\src\caching.rs" -Raw
$cachingContent = $cachingContent -replace "Ok\(CacheService::new\(config\.clone\(\)\)\)", "CacheService::new(config.clone())"
Set-Content "services\api-gateway\src\caching.rs" $cachingContent

$monitoringContent = Get-Content "services\api-gateway\src\monitoring.rs" -Raw
$monitoringContent = $monitoringContent -replace "Ok\(MonitoringService::new\(config\.clone\(\)\)\)", "MonitoringService::new(&config.clone())"
Set-Content "services\api-gateway\src\monitoring.rs" $monitoringContent

$loadBalancingContent = Get-Content "services\api-gateway\src\load_balancing.rs" -Raw
$loadBalancingContent = $loadBalancingContent -replace "Ok\(LoadBalancer::new\(config\.clone\(\), service_registry\)\)", "LoadBalancer::new(config.clone())"
Set-Content "services\api-gateway\src\load_balancing.rs" $loadBalancingContent

# Fix lib.rs to use Arc
$libContent = Get-Content "services\api-gateway\src\lib.rs" -Raw
$libContent = $libContent -replace "let load_balancer = load_balancing::init\(&config, service_registry\.clone\(\)\)\.await\?;", "let load_balancer = Arc::new(load_balancing::init(&config, service_registry.clone()).await?);"
$libContent = $libContent -replace "let auth_service = auth::init\(&config\)\.await\?;", "let auth_service = Arc::new(auth::init(&config).await?);"
$libContent = $libContent -replace "let rate_limiter = rate_limiting::init\(&config\)\.await\?;", "let rate_limiter = Arc::new(rate_limiting::init(&config).await?);"
$libContent = $libContent -replace "let cache = caching::init\(&config\)\.await\?;", "let cache = Arc::new(caching::init(&config).await?);"
Set-Content "services\api-gateway\src\lib.rs" $libContent

# Fix duplicate init function in service_discovery.rs
$serviceDiscoveryContent = Get-Content "services\api-gateway\src\service_discovery.rs" -Raw
$serviceDiscoveryContent = $serviceDiscoveryContent -replace "    /// Initialize service discovery`n    pub async fn init\(config: &Config\) -> Result<Self> {`n        Self::new\(config\.clone\(\)\)`n    }`n`n    /// Create a new service discovery service", "    /// Create a new service discovery service"
Set-Content "services\api-gateway\src\service_discovery.rs" $serviceDiscoveryContent

# Add init function at module level
$serviceDiscoveryContent = Get-Content "services\api-gateway\src\service_discovery.rs" -Raw
$serviceDiscoveryContent = $serviceDiscoveryContent -replace "impl ServiceDiscoveryService {", "/// Initialize service discovery`npub async fn init(config: &Config) -> Result<ServiceDiscoveryService> {`n    Ok(ServiceDiscoveryService::new(config.clone())?)`n}`n`nimpl ServiceDiscoveryService {"
Set-Content "services\api-gateway\src\service_discovery.rs" $serviceDiscoveryContent

Write-Host "Fixed remaining compilation errors"

# Fix Axum 0.7 API in all service templates
Write-Host "🔧 Fixing Axum 0.7 API in service templates..." -ForegroundColor Yellow

$services = @(
    "api-gateway",
    "user-management", 
    "inventory-service",
    "chat-service",
    "guild-service",
    "world-service",
    "matchmaking-service",
    "event-service",
    "content-management-service",
    "notification-service",
    "payment-service",
    "anti-cheat-service",
    "analytics-service"
)

foreach ($service in $services) {
    $mainRsPath = "services\$service\src\main.rs"
    if (Test-Path $mainRsPath) {
        Write-Host "  🔧 Fixing $service..." -ForegroundColor Cyan
        
        # Read the file
        $content = Get-Content $mainRsPath -Raw
        
        # Replace the old axum::Server code with new axum::serve code
        $oldCode = @"
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 $service server starting on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
"@

        $newCode = @"
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 $service server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
"@

        # Replace the content
        $content = $content -replace [regex]::Escape($oldCode), $newCode
        
        # Write back to file
        Set-Content -Path $mainRsPath -Value $content
        
        Write-Host "    ✅ Fixed $service" -ForegroundColor Green
    }
}

Write-Host "✅ All service templates fixed!" -ForegroundColor Green

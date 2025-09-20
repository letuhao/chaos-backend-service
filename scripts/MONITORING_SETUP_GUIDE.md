# 🎯 Chaos World Monitoring Setup Guide

## 📊 Current Status
- ✅ **Prometheus**: Running on http://localhost:9091
- ✅ **Grafana**: Running on http://localhost:3001  
- ✅ **CMS Metrics**: Available on http://localhost:9090/metrics
- ⚠️ **Grafana**: Needs manual setup (credentials issue)

## 🚀 Quick Setup Steps

### 1. Access Grafana
Open your browser and go to: **http://localhost:3001**

### 2. First Time Login
- **Username**: `admin`
- **Password**: `admin`
- If this doesn't work, try:
  - **Username**: `admin`  
  - **Password**: `admin123`
- Or check the Grafana logs for the actual password

### 3. Set Up Prometheus Datasource
1. In Grafana, go to **Configuration** → **Data Sources**
2. Click **Add data source**
3. Select **Prometheus**
4. Set **URL** to: `http://localhost:9091`
5. Click **Save & Test**

### 4. Create Your First Dashboard
1. Go to **Dashboards** → **New** → **New Dashboard**
2. Click **Add Panel**
3. In the query box, try these metrics:

#### Service Status
```
up{job=~"chaos-world-.*"}
```

#### Request Rate
```
rate(cms_request_duration_seconds_count[5m])
```

#### Response Time
```
histogram_quantile(0.95, rate(cms_request_duration_seconds_bucket[5m]))
```

#### Error Rate
```
rate(cms_errors_total[5m])
```

#### Active Connections
```
cms_active_connections
```

### 5. Pre-built Dashboard JSON
If you want to import a pre-built dashboard, here's the JSON:

```json
{
  "dashboard": {
    "id": null,
    "title": "Chaos World - Service Overview",
    "tags": ["chaos-world"],
    "timezone": "browser",
    "panels": [
      {
        "id": 1,
        "title": "Service Status",
        "type": "stat",
        "targets": [
          {
            "expr": "up{job=~\"chaos-world-.*\"}",
            "legendFormat": "{{job}}"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "color": {"mode": "thresholds"},
            "thresholds": {
              "steps": [
                {"color": "red", "value": 0},
                {"color": "green", "value": 1}
              ]
            }
          }
        },
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(cms_request_duration_seconds_count[5m])",
            "legendFormat": "CMS Requests/sec"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
      },
      {
        "id": 3,
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(cms_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          },
          {
            "expr": "histogram_quantile(0.50, rate(cms_request_duration_seconds_bucket[5m]))",
            "legendFormat": "50th percentile"
          }
        ],
        "gridPos": {"h": 8, "w": 24, "x": 0, "y": 8}
      },
      {
        "id": 4,
        "title": "Error Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(cms_errors_total[5m])",
            "legendFormat": "Errors/sec"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 16}
      },
      {
        "id": 5,
        "title": "Active Connections",
        "type": "graph",
        "targets": [
          {
            "expr": "cms_active_connections",
            "legendFormat": "Active Connections"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 16}
      }
    ],
    "time": {"from": "now-1h", "to": "now"},
    "refresh": "5s"
  }
}
```

## 🔧 Troubleshooting

### If Grafana won't start:
```bash
# Check if port 3001 is in use
netstat -ano | findstr ":3001"

# Kill any conflicting processes
taskkill /F /IM grafana-server.exe

# Restart Grafana
python setup_monitoring.py
```

### If Prometheus shows no data:
1. Check if CMS service is running: `curl http://localhost:8083/health`
2. Check Prometheus targets: http://localhost:9091/targets
3. Verify metrics: http://localhost:9090/metrics

### If you can't login to Grafana:
1. Check Grafana logs: `C:\ChaosWorld\monitoring\grafana\logs\grafana.log`
2. Look for the admin password in the logs
3. Or reset the admin password

## 📈 Available Metrics

Your CMS service exposes these metrics:
- `cms_active_connections` - Number of active connections
- `cms_cache_hits_total` - Cache hits
- `cms_cache_misses_total` - Cache misses  
- `cms_database_queries_total` - Database queries
- `cms_errors_total` - Total errors
- `cms_request_duration_seconds_*` - Request timing histograms
- `cms_requests_total` - Total requests

## 🎯 Next Steps

1. **Set up the datasource** (most important!)
2. **Create your first dashboard** with the metrics above
3. **Add more services** as you build them
4. **Set up alerts** for critical metrics
5. **Create custom dashboards** for different teams

## 🆘 Need Help?

If you're stuck:
1. Check the logs in `C:\ChaosWorld\monitoring\`
2. Run `python verify_monitoring.py` to check status
3. Run `python generate_test_data.py` to create test traffic
4. Check Prometheus directly: http://localhost:9091

Happy monitoring! 🎮📊

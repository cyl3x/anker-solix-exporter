# Anker Solix E1600 metrics exporter
This is a simple Prometheus exporter for the Anker Solix E1600 Solarbank.

## Usage
### Docker CLI
```bash
docker run -d -p 8080:8080 \
    --name anker-solix-exporter \
    -e ANKER_SOLIX_USERNAME=<username> \
    -e ANKER_SOLIX_PASSWORD=<password> \
    -e ANKER_SOLIX_SCENE_ID=<scene-id> \
    ghcr.io/cyl3x/anker-solix-exporter:latest
```

### Docker Compose
```yaml
services:
  anker-solix-exporter:
    image: ghcr.io/cyl3x/anker-solix-exporter:latest
    ports:
      - 8080:8080
    environment:
      ANKER_SOLIX_USERNAME: <username>
      ANKER_SOLIX_PASSWORD: <password>
      ANKER_SOLIX_SCENE_ID: <scene-id>
```

### Prometheus
```yaml
scrape_configs:
  - job_name: anker-solix-exporter
    scrape_interval: 5m
    scrape_timeout: 30s # timeout should be high, as data is fetched on scrape
    static_configs:
      - targets: ['anker-solix-exporter:8080']
```

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
    -v /tmp/anker-solix-exporter:/app \
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
    volumes:
      - /tmp/anker-solix-exporter:/app # for persistent token cache
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

## Exported metrics
| Metric | Description |
| ------ | ----------- |
| `anker_solix_home_load_power` | Home load power |
| `anker_solix_other_load_power` | Other load power |
| `anker_solix_grid_to_home_power` | Grid to home power |
| `anker_solix_photovoltaic_to_grid_power` | Photovoltaic to grid power |
| `anker_solix_home_charging_power` | Home charging power |
| `anker_solix_statistics_total_power` | Statistics total power |
| `anker_solix_statistics_total_co2` | Statistics total CO2 |
| `anker_solix_statistics_total_money` | Statistics total money |
| `anker_solix_solar_power_1` | Solar power 1 |
| `anker_solix_solar_power_2` | Solar power 2 |
| `anker_solix_solar_power_3` | Solar power 3 |
| `anker_solix_solar_power_4` | Solar power 4 |
| `anker_solix_solarbank_battery_power` | Solarbank power percent |
| `anker_solix_solarbank_charging_power` | Solarbank charging power |
| `anker_solix_solarbank_output_power` | Solarbank output power |
| `anker_solix_solarbank_photovoltaic_power` | Solarbank photovoltaic power |
| `anker_solix_solarbank_total_charging_power` | Solarbank total charging power |
| `anker_solix_solarbank_total_output_power` | Solarbank total output power |
| `anker_solix_solarbank_total_photovoltaic_power` | Solarbank total photovoltaic power |

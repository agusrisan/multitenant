# Deployment Guide

## Overview

This guide covers deploying the Multitenant Auth App to production using Docker or traditional server deployment.

---

## Prerequisites

- PostgreSQL 18+ database
- Domain name with SSL certificate (for production)
- Rust 1.75+ (for manual deployment)
- Docker & Docker Compose (for Docker deployment)
- Node.js 20+ (for frontend build)

---

## Environment Variables

Create a `.env.production` file based on `.env.production.example`:

```bash
# Database
DATABASE_URL=postgres://username:password@host:5432/database_name
DATABASE_MAX_CONNECTIONS=20
DATABASE_CONNECT_TIMEOUT=30

# Server
HOST=0.0.0.0
PORT=3000

# Secrets (CHANGE THESE!)
JWT_SECRET=your-super-secret-jwt-key-minimum-32-characters-long
JWT_ACCESS_EXPIRY=900
JWT_REFRESH_EXPIRY=604800

SESSION_SECRET=your-super-secret-session-key-minimum-32-characters-long
SESSION_EXPIRY=86400

CSRF_SECRET=your-super-secret-csrf-key-minimum-32-characters-long

# Application
RUST_ENV=production
RUST_LOG=info

# CORS
ALLOWED_ORIGINS=https://yourdomain.com,https://www.yourdomain.com
```

### Generating Secrets

Use OpenSSL to generate secure random secrets:

```bash
openssl rand -base64 48
```

---

## Docker Deployment (Recommended)

### 1. Build Docker Image

```bash
docker build -t multitenant-app:latest .
```

### 2. Configure Docker Compose

Update `docker-compose.yml` with production settings:

```yaml
services:
  postgres:
    image: postgres:18-alpine
    environment:
      POSTGRES_DB: multitenant_db
      POSTGRES_USER: ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: always

  app:
    image: multitenant-app:latest
    env_file:
      - .env.production
    ports:
      - "3000:3000"
    depends_on:
      - postgres
    restart: always
```

### 3. Run Migrations

```bash
docker-compose run app sqlx migrate run
```

### 4. Start Services

```bash
docker-compose up -d
```

### 5. Verify Deployment

```bash
curl http://localhost:3000/health
```

---

## Manual Deployment

### 1. Setup Database

```bash
# Create production database
createdb multitenant_db

# Set database URL
export DATABASE_URL=postgres://username:password@localhost:5432/multitenant_db

# Run migrations
sqlx migrate run
```

### 2. Build Frontend

```bash
cd resources
npm install
npm run build
cd ..
```

### 3. Build Backend

```bash
# Prepare SQLx for offline mode
cargo sqlx prepare

# Build for release
cargo build --release
```

### 4. Run Application

```bash
# Set environment variables
export RUST_ENV=production
export RUST_LOG=info
export JWT_SECRET=your-jwt-secret
export SESSION_SECRET=your-session-secret
export CSRF_SECRET=your-csrf-secret

# Run the binary
./target/release/multitenant
```

---

## Nginx Reverse Proxy Setup

### Configuration

Create `/etc/nginx/sites-available/multitenant`:

```nginx
upstream multitenant_app {
    server 127.0.0.1:3000;
}

server {
    listen 80;
    server_name yourdomain.com www.yourdomain.com;

    # Redirect HTTP to HTTPS
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com www.yourdomain.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/yourdomain.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # Security Headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-Frame-Options "DENY" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Gzip Compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css text/xml text/javascript application/javascript application/json;

    # Static Assets (with caching)
    location /assets/ {
        alias /path/to/app/resources/dist/assets/;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Proxy to App
    location / {
        proxy_pass http://multitenant_app;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### Enable Site

```bash
sudo ln -s /etc/nginx/sites-available/multitenant /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## SSL Certificate Setup (Let's Encrypt)

### Install Certbot

```bash
sudo apt update
sudo apt install certbot python3-certbot-nginx
```

### Obtain Certificate

```bash
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com
```

### Auto-renewal

Certbot automatically sets up renewal. Test it:

```bash
sudo certbot renew --dry-run
```

---

## Systemd Service (Manual Deployment)

Create `/etc/systemd/system/multitenant.service`:

```ini
[Unit]
Description=Multitenant Auth Application
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=appuser
Group=appuser
WorkingDirectory=/opt/multitenant
EnvironmentFile=/opt/multitenant/.env.production
ExecStart=/opt/multitenant/target/release/multitenant
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Enable and Start Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable multitenant
sudo systemctl start multitenant
sudo systemctl status multitenant
```

---

## Database Backup

### Automated Backup Script

Create `/opt/backup/backup_db.sh`:

```bash
#!/bin/bash
BACKUP_DIR="/opt/backups"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/multitenant_$DATE.sql"

# Create backup
pg_dump -U postgres multitenant_db > $BACKUP_FILE

# Compress backup
gzip $BACKUP_FILE

# Delete backups older than 7 days
find $BACKUP_DIR -name "*.sql.gz" -mtime +7 -delete

echo "Backup completed: $BACKUP_FILE.gz"
```

### Setup Cron Job

```bash
# Run daily at 2 AM
0 2 * * * /opt/backup/backup_db.sh
```

---

## Monitoring

### Application Logs

```bash
# Docker
docker-compose logs -f app

# Systemd
sudo journalctl -u multitenant -f
```

### Database Monitoring

```bash
# Check connection count
psql -U postgres -d multitenant_db -c "SELECT count(*) FROM pg_stat_activity;"

# Check database size
psql -U postgres -d multitenant_db -c "SELECT pg_size_pretty(pg_database_size('multitenant_db'));"
```

### Health Check Monitoring

Use a monitoring service to check `/health` endpoint:

```bash
# Example with cron
*/5 * * * * curl -f http://localhost:3000/health || echo "App is down!" | mail -s "Alert" admin@example.com
```

---

## Performance Tuning

### PostgreSQL Configuration

Edit `/etc/postgresql/18/main/postgresql.conf`:

```ini
# Connection Settings
max_connections = 100
shared_buffers = 256MB

# Query Performance
effective_cache_size = 1GB
maintenance_work_mem = 64MB
work_mem = 16MB

# WAL Settings
wal_buffers = 16MB
checkpoint_completion_target = 0.9
```

### Application Settings

Adjust in `.env.production`:

```bash
DATABASE_MAX_CONNECTIONS=20  # Adjust based on load
```

---

## Scaling

### Horizontal Scaling

1. **Load Balancer**: Use Nginx or HAProxy
2. **Multiple App Instances**: Run multiple containers/processes
3. **Database Connection Pooling**: Configure per instance
4. **Session Store**: Use Redis for shared sessions (future enhancement)

### Vertical Scaling

- Increase server resources (CPU, RAM)
- Optimize database (indexes, query performance)
- Enable database replication for read scaling

---

## Security Checklist

- [ ] Strong secrets generated and stored securely
- [ ] SSL/TLS enabled with valid certificate
- [ ] CORS configured with specific origins
- [ ] Security headers enabled
- [ ] Database backups automated
- [ ] Firewall configured (only ports 80, 443, 22 open)
- [ ] SSH key-based authentication
- [ ] Regular security updates applied
- [ ] Application logs monitored
- [ ] Rate limiting enabled
- [ ] Database credentials rotated regularly

---

## Troubleshooting

### Application Won't Start

```bash
# Check logs
docker-compose logs app
# or
sudo journalctl -u multitenant -n 100

# Verify environment variables
docker-compose run app env

# Check database connectivity
docker-compose run app psql $DATABASE_URL -c "SELECT 1;"
```

### Database Migration Issues

```bash
# Check migration status
sqlx migrate info

# Force migration
docker-compose run app sqlx migrate run --force
```

### Performance Issues

```bash
# Check database connections
docker-compose exec postgres psql -U postgres -c "SELECT * FROM pg_stat_activity;"

# Monitor resource usage
docker stats

# Check slow queries
docker-compose exec postgres psql -U postgres -c "SELECT * FROM pg_stat_statements ORDER BY mean_time DESC LIMIT 10;"
```

---

## Rollback Procedure

### Docker Deployment

```bash
# Stop current version
docker-compose down

# Restore previous image
docker tag multitenant-app:previous multitenant-app:latest

# Start services
docker-compose up -d
```

### Database Rollback

```bash
# Restore from backup
gunzip /opt/backups/multitenant_20250117_020000.sql.gz
psql -U postgres -d multitenant_db < /opt/backups/multitenant_20250117_020000.sql
```

---

## Production Deployment Checklist

- [ ] Environment variables configured
- [ ] Secrets generated and secured
- [ ] Database created and migrated
- [ ] Frontend built and optimized
- [ ] Backend built in release mode
- [ ] SSL certificate obtained
- [ ] Nginx configured and tested
- [ ] Firewall configured
- [ ] Systemd service created (if manual)
- [ ] Monitoring setup
- [ ] Backup automation configured
- [ ] Health check endpoint tested
- [ ] Application logs reviewed
- [ ] Performance testing completed
- [ ] Security audit performed

---

## Support & Resources

- **API Documentation**: `docs/api.md`
- **Development Guide**: `docs/development.md`
- **Architecture Documentation**: `docs/architecture.md`

---

## Production URLs

After deployment, your application will be available at:

- **Web App**: `https://yourdomain.com`
- **API**: `https://yourdomain.com/api`
- **Health Check**: `https://yourdomain.com/health`

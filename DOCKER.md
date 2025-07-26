# Docker Setup

This directory contains Docker Compose configuration to run the entire TinyRetro application stack with Traefik as a reverse proxy.

## Quick Start

1. **Start the application:**

   ```bash
   docker-compose up -d
   ```

2. **Access the application:**

   - Main application: http://localhost
   - Traefik dashboard: http://localhost:8080

3. **Stop the application:**
   ```bash
   docker-compose down
   ```

## Services

- **traefik**: Reverse proxy and load balancer
  - Routes requests to appropriate services
  - Handles SSL termination (when configured)
  - Dashboard available at port 8080
- **server**: Rust backend (Axum framework)
  - Handles API requests and WebSocket connections
  - Runs on internal port 3000
  - Accessible via `/api/*` and `/ws` paths
- **client**: SvelteKit frontend (served by Nginx)
  - Serves the static frontend application
  - Proxies API and WebSocket requests to the backend
  - Runs on internal port 80

## Production Deployment

For production deployment:

1. **Configure your domain:**
   - Copy `.env.example` to `.env`
   - Update `DOMAIN` and `ACME_EMAIL` variables
2. **Enable HTTPS:**

   - Uncomment the HTTPS-related labels in `docker-compose.yml`
   - Replace `localhost` with your actual domain
   - Traefik will automatically obtain Let's Encrypt certificates

3. **Security considerations:**
   - Disable Traefik API in production by removing `--api.insecure=true`
   - Configure proper firewall rules
   - Use Docker secrets for sensitive data

## Development

The `docker-compose.override.yml` file contains development-specific configurations that are automatically applied when running `docker-compose up`.

## Troubleshooting

- **Check logs:** `docker-compose logs [service-name]`
- **Restart services:** `docker-compose restart [service-name]`
- **Rebuild images:** `docker-compose up --build`
- **View Traefik routes:** Visit http://localhost:8080

## File Structure

```
├── docker-compose.yml          # Main compose configuration
├── docker-compose.override.yml # Development overrides
├── .env.example               # Environment variables template
├── client/
│   ├── Dockerfile            # Frontend build configuration
│   └── default.conf          # Nginx configuration
└── server/
    └── Dockerfile            # Backend build configuration
```

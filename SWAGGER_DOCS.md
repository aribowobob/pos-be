## API Documentation with OpenAPI/Swagger

This project includes Swagger UI for interactive API documentation in the development environment only.

### Accessing the Swagger UI

When running in development mode, Swagger UI is available at:

```
http://localhost:8080/swagger-ui/
```

This interactive documentation allows you to:

- View all API endpoints
- See request parameters and response schemas
- Test API calls directly from the browser
- Understand authentication requirements

### Development vs Production

- **Development Mode**: Swagger UI is enabled
- **Production Mode**: Swagger UI is disabled for security reasons

### Running in Different Modes

Set the environment variable to control which mode the application runs in:

```bash
# Enable Swagger UI (development)
export ENVIRONMENT=development
cargo run

# Disable Swagger UI (production)
export ENVIRONMENT=production
cargo run

# For local development with auto-reload
cargo watch -x run
```

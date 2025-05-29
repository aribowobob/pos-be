# POS Backend

A Point of Sale (POS) backend system built with Rust and Actix-web, featuring Google OAuth2 authentication.

## Features

- REST API built with Actix-web
- Google OAuth2 integration for user authentication
- JWT token-based authentication
- PostgreSQL database integration
- Automated database migration through SQL files
- Structured error handling
- Health check endpoints
- Role-based access control

## Project Structure

```
src/
├── errors/           # Custom error definitions
├── handlers/         # Request handlers
│   ├── auth.rs       # Authentication handlers
│   ├── user.rs       # User-related handlers
│   └── mod.rs
├── middleware/       # Custom middlewares
├── models/           # Data structures
│   ├── app_state.rs  # Application state
│   ├── auth.rs       # Auth-related models
│   ├── response.rs   # API response structures
│   ├── user.rs       # User & Store models
│   └── mod.rs
├── routes/           # API route definitions
│   ├── auth.rs       # Auth routes
│   ├── health.rs     # Health check routes
│   ├── orders.rs     # Order management routes
│   ├── products.rs   # Product management routes
│   ├── user.rs       # User routes
│   └── mod.rs
├── services/         # Business logic
│   ├── auth.rs       # Auth services
│   ├── user_service.rs # User-related services
│   └── mod.rs
└── main.rs           # Application entry point
```

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- PostgreSQL
- Google Cloud Console account (for OAuth2 configuration)

## Environment Variables

Create a `.env` file in the root directory with:

```env
# Database configuration
DATABASE_URL=postgresql://username:password@localhost:5432/pos_db

# Authentication
GOOGLE_CLIENT_ID=your_client_id_here
GOOGLE_CLIENT_SECRET=your_client_secret_here
JWT_SECRET=your_jwt_secret_here

# Application settings
ENVIRONMENT=development
FRONTEND_URL=http://localhost:3000
PORT=8080
```

**Security Warning:** Do not share your `.env` file or expose it publicly. It contains sensitive information that should be kept secret.

## Installation

1. Install Rust and Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install PostgreSQL and create database

```bash
# For Ubuntu/Debian
sudo apt install postgresql

# For macOS
brew install postgresql

# Create database
createdb pos_db
```

3. Set up the project

```bash
# Clone the repository
git clone https://github.com/yourusername/pos-be.git
cd pos-be

# Copy environment template
cp .env.example .env

# Configure your .env file with appropriate values
```

4. Development

```bash
# Run in development mode with auto-reload (if cargo-watch is installed)
cargo watch -x run

# Or run normally
cargo run

# Running on different port (if default port 8080 is in use)
PORT=8081 cargo run

# Set up the database tables automatically (adjust port number if changed)
curl http://localhost:8080/db-migration
# Or if using custom port
curl http://localhost:8081/db-migration
```

5. Build for production

```bash
cargo build --release
```

## API Documentation

The API provides the following endpoints:

- **Auth**: `/api/auth/*` - Authentication endpoints
- **Users**: `/api/users/*` - User management
- **Products**: `/api/products/*` - Product management
- **Orders**: `/api/orders/*` - Order management
- **Health**: `/health` - Health check endpoint
- **Database Migration**: `/db-migration` - Create or migrate database tables

## Database Migration System

This application features an automated database migration system that:

1. Reads SQL files from the `src/db_migration/` directory
2. Executes them in alphabetical order (e.g., 001_file.sql, 002_file.sql)
3. Handles PL/pgSQL blocks with dollar-quoted strings properly
4. Continues execution even when tables already exist
5. Provides detailed logging for successful and failed migrations

### Using the Migration System

The migration endpoint is public and can be accessed without authentication:

```bash
curl http://localhost:8080/db-migration
```

### Adding New Migrations

To add new database changes:

1. Create a new SQL file in the `src/db_migration/` directory with a numeric prefix (e.g., `004_new_feature.sql`)
2. Use standard SQL and/or PL/pgSQL statements
3. For PL/pgSQL blocks, use named delimiters:
   ```sql
   DO $block$
   BEGIN
     -- Your PL/pgSQL code here
   END $block$;
   ```
4. Use conditional statements to avoid errors when objects already exist:

   ```sql
   -- For tables
   CREATE TABLE IF NOT EXISTS my_table (...);

   -- For columns
   DO $block$
   BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'my_table' AND column_name = 'new_column'
    ) THEN
        ALTER TABLE my_table ADD COLUMN new_column TEXT;
    END IF;
   END $block$;
   ```

The migration system automatically handles most common errors like "table already exists" or "duplicate key violations" and continues processing other statements.

## Dependencies

Main dependencies used in this project:

- `actix-web`: Web framework
- `sqlx`: Async SQL toolkit
- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `jsonwebtoken`: JWT authentication
- `reqwest`: HTTP client for OAuth2
- `chrono`: Date and time utilities
- `dotenv`: Environment variable management

## Deployment

This project uses GitHub Actions for CI/CD deployment to a Digital Ocean droplet. The workflow is defined in `.github/workflows/prod.yml` and is triggered when a tag with format `v*` is pushed to the repository.

### GitHub Actions Setup

The workflow handles:

1. Building a Docker image
2. Pushing the image to Docker Hub
3. Deploying the application to a Digital Ocean droplet using SSH
4. Setting up and configuring the application environment

### Required GitHub Secrets

The following secrets need to be configured in your GitHub repository:

- `DOCKER_USERNAME`: Docker Hub username
- `DOCKER_PASSWORD`: Docker Hub password
- `DROPLET_PASSWORD`: Password for the Digital Ocean droplet
- `POSTGRES_PASSWORD`: Password for the PostgreSQL database
- `DO_TOKEN`: Digital Ocean API token
- `SSH_PRIVATE_KEY`: (Optional) SSH private key for secure connection to the droplet

### Required GitHub Variables

- `DROPLET_IP`: IP address of the Digital Ocean droplet

### How to Deploy

To deploy a new version:

```bash
# Tag a new version
git tag v1.0.0

# Push the tag to GitHub
git push origin v1.0.0
```

The GitHub Actions workflow will automatically:

1. Build the Docker image
2. Tag it with the version number
3. Deploy it to your Digital Ocean droplet

## License

This project is licensed under the MIT License - see the LICENSE file for details.

# POS Backend

A Point of Sale (POS) backend system built with Rust and Actix-web, featuring Google OAuth2 authentication.

## Features

- REST API with Actix-web
- Google OAuth2 Authentication
- JWT Token Generation
- CORS Support

## Project Structure

```
src/
├── errors/           # Custom error definitions
├── handlers/         # Request handlers
│   ├── auth.rs      # Authentication handlers
│   ├── user.rs      # User-related handlers
│   └── mod.rs
├── middleware/       # Custom middlewares
├── models/          # Data structures
│   ├── app_state.rs # Application state
│   ├── auth.rs      # Auth-related models
│   ├── response.rs  # API response structures
│   ├── user.rs      # User & Store models
│   └── mod.rs
├── routes/          # API route definitions
│   ├── auth.rs      # Auth routes
│   ├── health.rs    # Health check routes
│   ├── orders.rs    # Order management routes
│   ├── products.rs  # Product management routes
│   ├── user.rs      # User routes
│   └── mod.rs
├── services/        # Business logic
│   ├── auth.rs      # Auth services
│   ├── user_service.rs # User-related services
│   └── mod.rs
└── main.rs         # Application entry point
```

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- PostgreSQL (for future database integration)
- Google Cloud Console account (for OAuth2)

## Environment Variables

Create a `.env` file in the root directory with:

```env
DATABASE_URL=postgresql://username:your_password@localhost:5432/db_name
GOOGLE_CLIENT_ID=your_client_id_here
GOOGLE_CLIENT_SECRET=your_client_secret_here
JWT_SECRET=your_jwt_secret_here
ENVIRONMENT=development | production
FRONTEND_URL=your_frontend_url
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
# DATABASE_URL=postgresql://postgres:postgresql9!@localhost:5432/pos_db
# GOOGLE_CLIENT_ID=your_client_id
# GOOGLE_CLIENT_SECRET=your_client_secret
# JWT_SECRET=your_secret
# ENVIRONMENT=development
# FRONTEND_URL=http://localhost:3000
```

4. Development
```bash
# Run in development mode with auto-reload
cargo dev

# Or run normally
cargo run
```

The server will start at `http://localhost:8080`

## Dependencies

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "chrono", "json"] }
actix-web = "4.3"
actix-cors = "0.6.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"
jsonwebtoken = "8.3"
reqwest = { version = "0.11", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1.28", features = ["full"] }
oauth2 = "4.4"
log = "0.4"
env_logger = "0.10"
```

## License

MIT

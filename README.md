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
├── handlers/           # Request handlers
│   ├── mod.rs
│   └── auth.rs        # Authentication handlers
├── models/            # Data structures
│   ├── mod.rs
│   └── auth.rs        # Auth-related models
├── services/          # Business logic
│   ├── mod.rs
│   └── auth.rs        # Auth service implementation
└── main.rs           # Application entry point
```

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- PostgreSQL (for future database integration)
- Google Cloud Console account (for OAuth2)

## Environment Variables

Create a `.env` file in the root directory with:

```env
DATABASE_URL=postgresql://postgres:postgresql9!@localhost:5432/pos_db
GOOGLE_CLIENT_ID=your_client_id_here
GOOGLE_CLIENT_SECRET=your_client_secret_here
JWT_SECRET=your_jwt_secret_here
```

## Installation

1. Clone the repository
```bash
git clone [your-repository-url]
cd pos-be
```

2. Install dependencies and build
```bash
cargo build
```

3. Run the application
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

## API Endpoints

### Public Endpoints
- `GET /` - Welcome message

### Authentication Endpoints
- `POST /auth/google` - Google OAuth2 authentication
  - Request body: `{ "token": "google-id-token" }`
  - Response: 
    ```json
    {
      "token": "jwt-token",
      "user": {
        "email": "user@example.com",
        "name": "User Name",
        "picture": "profile-picture-url"
      }
    }
    ```

## Dependencies

- actix-web - Web framework
- actix-cors - CORS middleware
- serde - Serialization/Deserialization
- jsonwebtoken - JWT implementation
- reqwest - HTTP client
- dotenv - Environment configuration
- chrono - Time utilities

## License

MIT

# POS Backend

A Point of Sale (POS) backend system built with Rust and Actix-web.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- PostgreSQL (optional, if using database)

## Installation

1. Clone the repository
```bash
git clone [your-repository-url]
cd pos-be
```

2. Create `.env` file in root directory (optional)
```bash
cp .env.example .env
```

3. Build the project
```bash
cargo build
```

## Running the Application

To run in development mode:
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

## API Endpoints

- `GET /` - Welcome message endpoint

## Project Structure

```
pos-be/
├── src/
│   └── main.rs
├── Cargo.toml
├── .env
└── README.md
```

## Development

This project uses:
- actix-web - Web framework
- serde - Serialization/Deserialization
- dotenv - Environment variable management

## License

MIT

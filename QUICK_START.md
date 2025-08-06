# Quick Start Guide for Developers

## 🚀 Before You Start Coding

1. **Read the Development Guide**: [DEVELOPMENT_GUIDE.md](./DEVELOPMENT_GUIDE.md)
2. **Check your code**: Run `./check_code.sh` before pushing
3. **Test Docker compatibility**: Run `cargo build --release`

## 🤖 Working with AI Code Generation

**Always include this in your AI prompts:**

```
IMPORTANT: Use sqlx::query() not sqlx::query!() for Docker compatibility.
Use .bind() for parameters and proper ServiceError handling.
```

See [AI_PROMPTS.md](./AI_PROMPTS.md) for complete templates.

## ⚡ Quick Commands

```bash
# Check for issues
./check_code.sh

# Test compilation
cargo check
cargo build --release

# Run the application
cargo run

# Run with logs
RUST_LOG=debug cargo run
```

## 🆘 Emergency Fix for SQLx Errors

If you see "set `DATABASE_URL` to use query macros online":

1. Find the line with `sqlx::query!`
2. Replace with `sqlx::query` + `.bind()`
3. Test with `cargo build --release`
4. Push changes

Example fix:

```rust
// ❌ This breaks Docker builds
sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)

// ✅ This works in Docker
sqlx::query("SELECT * FROM users WHERE id = $1").bind(user_id)
```

## 📁 Important Files

- `DEVELOPMENT_GUIDE.md` - Complete development guidelines
- `AI_PROMPTS.md` - Templates for AI code generation
- `check_code.sh` - Code quality checker
- `Cargo.toml` - Has important SQLx configuration notes
- `README.md` - Main project documentation

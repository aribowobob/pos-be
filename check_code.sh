#!/bin/bash

# Code Quality Check Script for POS Backend
# This script checks for common issues that can break Docker builds

echo "🔍 Checking for potential SQLx issues..."
echo "=================================================="

# Check for problematic sqlx::query! usage
echo "1. Checking for sqlx::query! (compile-time queries):"
if grep -r "sqlx::query!" src/ 2>/dev/null; then
    echo "❌ FOUND: sqlx::query! usage detected!"
    echo "   These should be converted to sqlx::query() for Docker compatibility"
    echo "   See DEVELOPMENT_GUIDE.md for how to fix this"
else
    echo "✅ No sqlx::query! found - Good!"
fi

echo ""

# Check for SQLX_OFFLINE in Dockerfile
echo "2. Checking Dockerfile for SQLX_OFFLINE:"
if grep -q "SQLX_OFFLINE" Dockerfile 2>/dev/null; then
    echo "❌ FOUND: SQLX_OFFLINE in Dockerfile"
    echo "   Remove this unless you have proper offline setup"
else
    echo "✅ No SQLX_OFFLINE in Dockerfile - Good!"
fi

echo ""

# Check for missing FromRow imports
echo "3. Checking for proper SQLx imports:"
missing_imports=0
for file in $(find src/ -name "*.rs" -exec grep -l "sqlx::query" {} \;); do
    if ! grep -q "use sqlx::" "$file"; then
        echo "⚠️  $file: Missing SQLx imports"
        missing_imports=1
    fi
done

if [ $missing_imports -eq 0 ]; then
    echo "✅ SQLx imports look good!"
fi

echo ""

# Test compilation
echo "4. Testing compilation:"
echo "Running 'cargo check'..."
if cargo check --quiet; then
    echo "✅ Code compiles successfully!"
else
    echo "❌ Compilation failed - check the errors above"
fi

echo ""
echo "=================================================="
echo "Check complete!"
echo ""
echo "💡 Tips:"
echo "- Always run 'cargo build --release' before pushing"
echo "- See DEVELOPMENT_GUIDE.md for SQLx best practices"
echo "- Use AI_PROMPTS.md templates when requesting AI code"

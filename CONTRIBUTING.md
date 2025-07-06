# Contributing to skimtoken

Thank you for your interest in contributing to skimtoken! We welcome contributions that improve accuracy while maintaining our core principles of minimal memory usage and fast startup times.

## Ways to Contribute

### 1. Improve Accuracy
- Test on your language/domain and report accuracy
- Submit optimized parameters for specific languages
- Propose better statistical models

### 2. Enhance Performance
- Optimize for lower memory usage
- Propose changes to speed up tokenization
- Suggest efficient algorithms or data structures

### 3. Report Issues
- Accuracy problems with specific text types
- Performance regressions
- API usability feedback

### 4. Documentation
- Usage examples for different scenarios
- Tutorials for specific use cases
- Clarifications and corrections

## Development Setup

```bash
# Clone the repository
git clone https://github.com/masaishi/skimtoken
cd skimtoken

# Install dependencies with uv
uv sync

# Build development version
uv run maturin dev --features python

# Run tests
cargo test
uv run pytest
```

## Testing Your Changes

### 1. Run the Test Suite
```bash
# Rust tests
cargo test

# Python tests
uv run pytest

# Benchmarks
uv run scripts/benchmark.py
```

### 2. Check Code Quality
```bash
# Python
uv run ruff format && uv run ruff check --fix && uv run pyright

# Rust
cargo fmt && cargo clippy -- -D warnings
```

## Submitting Changes

### 1. Code Contributions

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Push and create a pull request

### 2. Commit Message Format

Follow conventional commits:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `test:` Test additions/changes
- `perf:` Performance improvements
- `refactor:` Code refactoring

## Questions?

Feel free to:
- Open an issue for discussion
- Ask in pull request comments
- Reach out to maintainers

Thank you for helping make skimtoken better!